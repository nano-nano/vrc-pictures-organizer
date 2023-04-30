import { invoke } from '@tauri-apps/api';
import { useCallback, useEffect, useState } from 'react';

export type Setting = {
  interval: number;
  date_line: string;
};

function useSetting() {
  const [interval, setInterval] = useState(10);
  const onChangeInterval = useCallback(
    (val: number | '') => setInterval(val === '' ? 10 : val),
    []
  );
  const [dateLineTime, setDateLineTime] = useState('12:00');
  const onChangeDateLineTime = useCallback(
    (val: string) => setDateLineTime(val),
    []
  );

  const isErrorDateLineTime = dateLineTime === '';

  const [isSaving, setIsSaving] = useState(false);

  const onClickSave = useCallback(() => {
    setIsSaving(true);
    const currentSetting: Setting = {
      interval: interval,
      date_line: dateLineTime,
    };
    invoke('save_setting_for_screen', currentSetting)
      .then(() => console.debug('finish'))
      .finally(() => {
        setIsSaving(false);
      });
  }, [dateLineTime, interval]);

  useEffect(() => {
    invoke<Setting>('get_setting_for_screen').then((setting) => {
      setInterval(setting.interval);
      setDateLineTime(setting.date_line);
    });
  }, []);

  return {
    interval,
    onChangeInterval,
    dateLineTime,
    onChangeDateLineTime,
    isErrorDateLineTime,
    onClickSave,
    isSaving,
  } as const;
}

export default useSetting;
