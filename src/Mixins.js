import { fs } from '@tauri-apps/api'

export default {
  methods: {
    async getDirectorysFromPath(path){
      let validEntrys = []
      const pathEntrys = await fs.readDir(path)        
      pathEntrys.forEach(entry => {
        if(entry.children){
          validEntrys.push(entry)
        }
      })
      return validEntrys
    },
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