import { invoke } from '@tauri-apps/api';
import { useState } from 'react';

function useLog() {
  const [logs, setLogs] = useState<string[]>([]);

  const formattedLogs = logs.join('\n');

  const fetchLog = async () => {
    invoke<string[]>('get_log_for_screen').then((logs) => setLogs(logs));
  };

  return { fetchLog, logs, formattedLogs } as const;
}

export default useLog;
