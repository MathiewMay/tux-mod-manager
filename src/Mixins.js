import { fs,  } from '@tauri-apps/api'

export default {
  methods: {
    async pathExists(path) {
      const pathDirExists = await fs.readDir(path).catch((reason) => {return true})
      if(pathDirExists != true){
        return true
      }else{
        return false
      }
    },
  },
};