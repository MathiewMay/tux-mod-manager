use std::time::Duration;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::{BufRead, BufReader, BufWriter, Seek, SeekFrom, Write};

use url::Url;
use reqwest::header::{self, HeaderMap, HeaderValue};
use reqwest::blocking::Client;
use failure::{format_err, Fallible};

use crate::mod_downloader::utils::{decode_percent_coded_string, get_file_handle};
use crate::mod_downloader::core::{Config, EventsHandler, HttpDownload};

pub struct ProgressTracker {
    pub filename: String,
    pub length: Option<u64>,
    pub current: Option<u64>
}

impl ProgressTracker {
    fn inc(&mut self, by: u64) {
        match self.current {
            Some(value) => {
                let new_value = value + by;
                self.current = Some(new_value);
            }
            None => {
                self.current = Some(by);
            }
        }
    }
    pub fn get_prog(&mut self) -> Option<(u64, u64)> {
        match self.length {
            Some(len) => {
                match self.current {
                    Some(cur) => {
                        Some((len, cur))
                    },
                    None => {
                        Some((len, 0u64))
                    }
                }
            },
            None => { None }
        }
    }
}

pub fn http_download(url: Url, save_path: PathBuf, progress_tracker: ProgressTracker, resume_download: bool, concurrent_download: bool, version: &str) -> Fallible<()> {
    let user_agent = format!("TMM/{}", &version);
    let timeout = 30u64;
    let num_workers = 8usize;
    let headers = request_headers(&url, timeout, "TMM/0.1.0")?;
    let filename = gen_filename(&url, Some(&headers));

    let content_len = if let Some(val) = headers.get("Content-Length") {
        val.to_str()?.parse::<u64>().unwrap_or(0)
    } else {
        0u64
    };

    let headers = prep_headers(&filename, resume_download, &user_agent)?;

    let state_file_exists = Path::new(&format!("{}.st", filename)).exists();
    let chunk_size = 512_000u64;

    let chunk_offsets = 
        if state_file_exists && resume_download && concurrent_download && content_len != 0 {
            Some(get_resume_chunk_offsets(&filename, content_len, chunk_size)?)
        } else {
            None
        };

    let bytes_on_disk = if resume_download {
        calc_bytes_on_disk(&filename)?
    } else {
        None
    };

    let conf = Config {
        user_agent: user_agent.clone(),
        resume: resume_download,
        headers,
        file: filename.clone(),
        save_path: save_path.clone(),
        timeout,
        concurrent: concurrent_download,
        max_retries: 100,
        num_workers,
        bytes_on_disk,
        chunk_offsets,
        chunk_size,
    };

    let mut client = HttpDownload::new(url.clone(), conf.clone());
    let events_handler = DefaultEventsHandler::new(&filename, &save_path.to_str().unwrap(), progress_tracker, resume_download, concurrent_download)?;
    client.events_hook(events_handler).download()?;
    Ok(())
}

fn request_headers(url: &Url, timeout: u64, ua: &str) -> Fallible<HeaderMap> {
    // let mut url_string = "".to_string();
    // String::clone_from(&mut url_string, &url.as_ref().to_string());
    // let copy: Url = Url::parse(url_string.as_str()).unwrap();
    // let mut file = std::fs::File::create("image.png").unwrap();
    // println!("{}, {}", &url, &copy);
    // let result = spawn_blocking(move || {reqwest::blocking::get(copy.as_ref()).unwrap().copy_to(&mut file).unwrap()});
    let response = Client::new()
        .get(url.as_ref())
        .timeout(Duration::from_secs(timeout))
        .header(header::USER_AGENT, HeaderValue::from_str(ua)?)
        .header(header::ACCEPT, HeaderValue::from_str("*/*")?)
        .send()?;
    Ok(response.headers().clone())
}

fn gen_filename(url: &Url, headers: Option<&HeaderMap>) -> String {
    let content_disposition = headers
        .and_then(|hdrs| hdrs.get(header::CONTENT_DISPOSITION))
        .and_then(|val| {
            let val = val.to_str().unwrap_or("");
            if val.contains("filename=") {
                Some(val)
            } else {
                None
            }
        })
        .and_then(|val| {
            let x = val
                .rsplit(';')
                .nth(0)
                .unwrap_or("")
                .rsplit('=')
                .nth(0)
                .unwrap_or("")
                .trim_start_matches('"')
                .trim_end_matches('"');
            if !x.is_empty() {
                Some(x.to_string())
            } else {
                None
            }
        });
    match content_disposition {
        Some(val) => val,
        None => {
            let name = &url.path().split('/').last().unwrap_or("");
            if !name.is_empty() {
                match decode_percent_coded_string(name) {
                    Ok(val) => val,
                    _ => name.to_string(),
                }
            } else {
                "index.html".to_owned()
            }
        }
    }
}

fn prep_headers(fname: &str, resume: bool, user_agent: &str) -> Fallible<HeaderMap> {
    let bytes_on_disk = calc_bytes_on_disk(fname)?;
    let mut headers = HeaderMap::new();
    if let Some(bcount) = bytes_on_disk {
        if resume {
            let byte_range = format!("bytes={}-", bcount);
            headers.insert(header::RANGE, byte_range.parse()?);
        }
    }

    headers.insert(header::USER_AGENT, user_agent.parse()?);

    Ok(headers)
}

fn calc_bytes_on_disk(fname: &str) -> Fallible<Option<u64>>{
    //.st is a state file, use it if possible
    let st_fname = format!("{}.st", fname);
    if Path::new(&st_fname).exists() {
        let input = fs::File::open(st_fname)?;
        let buf = BufReader::new(input);
        let mut byte_count: u64 = 0;
        for line in buf.lines() {
            let num_of_bytes = line?
                .split(':')
                .nth(0)
                .ok_or_else(|| format_err!("failed to split state file line"))?
                .parse::<u64>()?;
            byte_count += num_of_bytes;
        }
        return Ok(Some(byte_count));
    }
    match fs::metadata(fname) {
        Ok(metadata) => Ok(Some(metadata.len())),
        _ => Ok(None),
    }
}

fn get_resume_chunk_offsets(filename: &str, content_len: u64, chunk_size: u64) -> Fallible<Vec<(u64, u64)>> {
    let st_fname = format!("{}.st", filename);
    let input = fs::File::open(st_fname)?;
    let buf = BufReader::new(input);
    let mut downloaded = vec![];
    for line in buf.lines() {
        let l = line?;
        let l = l.split(':').collect::<Vec<_>>();
        let n = (l[0].parse::<u64>()?, l[1].parse::<u64>()?);
        downloaded.push(n);
    }
    downloaded.sort_by_key(|a| a.1);
    let mut chunks = vec![];

    let mut i: u64 = 0;
    for (bc, offset) in downloaded {
        if i == offset {
            i = offset + bc;
        } else {
            chunks.push((i, offset -1));
            i = offset + bc;
        }
    }

    while (content_len - i) > chunk_size {
        chunks.push((i, i + chunk_size - 1));
        i += chunk_size;
    }

    chunks.push((i, content_len));

    Ok(chunks)
}

pub struct DefaultEventsHandler {
    prog_tracker: Option<ProgressTracker>,
    bytes_on_disk: Option<u64>,
    filename: String,
    save_path: String,
    file: BufWriter<fs::File>,
    st_file: Option<BufWriter<fs::File>>,
    server_supports_resume: bool
}

impl DefaultEventsHandler {
    pub fn new(
        filename: &str,
        save_path: &str,
        progress_tracker: ProgressTracker,
        resume: bool,
        concurrent: bool
    ) -> Fallible<DefaultEventsHandler> {
        let st_file = if concurrent {
            Some(BufWriter::new(get_file_handle(
                &format!("{}.st", filename), save_path, resume, true
            )?))
        } else {
            None
        };
        Ok(DefaultEventsHandler {
            prog_tracker: Some(progress_tracker),
            bytes_on_disk: calc_bytes_on_disk(filename)?,
            filename: filename.to_owned(),
            save_path: save_path.to_owned(),
            file: BufWriter::new(get_file_handle(&filename, save_path, resume, !concurrent)?),
            st_file,
            server_supports_resume: false
        })
    }

    pub fn create_prog_tracker(&mut self, length: Option<u64>) {
        let byte_count = if self.server_supports_resume {
            self.bytes_on_disk
        } else {
            None
        };
        if let Some(len) = length {
            let exact = len;
            println!("Length: {}", exact);
        } else {
            println!("Length: unknown");
        }

        let mut prog_tracker = ProgressTracker {
            filename: self.filename.clone(),
            length,
            current: None
        };
        if let Some(count) = byte_count {
            prog_tracker.inc(count);
        }
        self.prog_tracker = Some(prog_tracker);
    }
}

impl EventsHandler for DefaultEventsHandler {
    fn on_headers(&mut self, headers: HeaderMap) {
        let content_type = if let Some(val) = headers.get(header::CONTENT_TYPE) {
            val.to_str().unwrap_or("")
        } else {
            ""
        };
        println!("Type: {}", content_type);
        println!("Saving to: {}", &self.filename);
    }

    fn on_server_supports_resume(&mut self) {
        self.server_supports_resume = true;
    }

    fn on_content(&mut self, content: &[u8]) -> Fallible<()> {
        let byte_count = content.len() as u64;
        self.file.write_all(content)?;
        if let Some(ref mut b) = self.prog_tracker {
            b.inc(byte_count);
        }
        Ok(())
    }

    fn on_concurrent_content(&mut self, content: (u64, u64, &[u8])) -> Fallible<()> {
        let (byte_count, offset, buf) = content;
        self.file.seek(SeekFrom::Start(offset))?;
        self.file.write_all(buf)?;
        self.file.flush()?;
        
        if let Some(ref mut b) = self.prog_tracker {
            b.inc(byte_count);
        }
        
        if let Some(ref mut file) = self.st_file {
            writeln!(file, "{}:{}", byte_count, offset)?;
            match file.flush() {
                Err(error) => {
                    eprintln!("Failed to flush file (concurrent download); {}", error);
                },
                Ok(_) => {}
            }
        }

        Ok(())
    }

    fn on_resume_download(&mut self, bytes_on_disk: u64) {
        self.bytes_on_disk = Some(bytes_on_disk);
    }

    fn on_finish(&mut self) {
        let st_file = format!("{}/{}.st", self.save_path.as_str(), self.filename);
        fs::remove_file(st_file);
    }

    fn on_max_retries(&mut self) {
        println!("Max retries exceeded, quitting!");
        match self.file.flush() {
            _ => {}
        }
        if let Some(ref mut file) = self.st_file {
            match file.flush() {
                _ => {}
            }
        }
        ::std::process::exit(0);
    }

    fn on_failure_status(&self, status_code: i32) {
        if status_code == 416 {
            println!("The file is already fully retrieved, nothing to do.");
        }
    }
}