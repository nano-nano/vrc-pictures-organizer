import IpcApi from '../preload';

declare global {
    interface Window {
        ipcApi: IpcApi;
    }
    const __static: string;
}

