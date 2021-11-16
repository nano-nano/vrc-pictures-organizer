import { contextBridge, ipcRenderer } from 'electron';
import { IpcChannel } from './constants';

export class IpcApi {
  public static readonly KEY = 'ipcApi';

  public getFolderPath = async () => {
    return ipcRenderer.invoke(IpcChannel.CHANNEL_GET_FOLDER_PATH, null) as Promise<string>;
  }
  public registerOnFinishFileOrganizeCallback = (cb: (successCount: number, failCount: number) => void) => {
    ipcRenderer.on(IpcChannel.CHANNEL_ON_FINISH_FILE_ORGANIZE, (_, arg) => cb(arg.successCount, arg.failCount));
  }
  public execFileOrganize = (folderPath: string) => {
    ipcRenderer.send(IpcChannel.CHANNEL_EXEC_FILE_ORGANIZE, { folderPath: folderPath });
  }
  public saveSettingsFile = (data: any, cb: (isSucceeded: boolean) => void) => {
    ipcRenderer.once(IpcChannel.CHANNEL_ON_SAVE_SETTINGS, (event, arg) => cb(arg.isSucceeded));
    ipcRenderer.send(IpcChannel.CHANNEL_SAVE_SETTINGS, data);
  }
}

contextBridge.exposeInMainWorld(
  IpcApi.KEY,
  new IpcApi()
);