import { useState } from 'react';
import { commands } from '../../../bindings';

export default function useAppLog() {
  const [logs, setLogs] = useState<string[]>([]);

  const formattedLogs = logs.join('\n');

  function fetchAppLog() {
    commands.fetchAppProcessLog().then((logs) => {
      if (logs.status == 'ok') {
        console.debug(logs.data);
        setLogs(logs.data);
      }
    });
  }

  return { fetchAppLog, formattedLogs } as const;
}
