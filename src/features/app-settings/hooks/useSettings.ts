import { notifications } from '@mantine/notifications';
import { useEffect, useState } from 'react';
import { commands } from '../../../bindings';
import { NOTIFICATION_MODE_VALUES, NotificationModeValue } from '../constants/notificationMode';

export type SettingsStruct = {
  interval: number;
  dateLine: string;
  notificationMode: NotificationModeValue;
};

export default function useSettings() {
  const [settings, setSettings] = useState<SettingsStruct>({
    interval: 60,
    dateLine: '12:00',
    notificationMode: 'onSuccess',
  });

  const isValidValue = 10 <= settings.interval && settings.interval <= 3600;

  function onChangeInterval(newVal: number) {
    setSettings((prev) => ({ ...prev, interval: newVal }));
  }

  function onChangeDateLine(newVal: string) {
    setSettings((prev) => ({ ...prev, dateLine: newVal }));
  }

  function onChangeNotificationMode(newVal: string | null) {
    if (newVal !== null && NOTIFICATION_MODE_VALUES.includes(newVal)) {
      setSettings((prev) => ({ ...prev, notificationMode: newVal as NotificationModeValue }));
    }
  }

  async function saveSettings() {
    commands
      .saveSettingsToFile({
        interval_sec: settings.interval,
        date_line: settings.dateLine,
        notification_mode: settings.notificationMode,
      })
      .then((res) => {
        if (res.status === 'ok') {
          notifications.show({
            title: '保存成功',
            message: '設定を保存しました',
            withBorder: true,
            withCloseButton: false,
          });
        } else {
          notifications.show({
            title: '保存失敗',
            message: '設定を保存できませんでした',
            color: 'red',
            withBorder: true,
            withCloseButton: false,
          });
        }
      });
  }

  useEffect(() => {
    commands.loadSettingsFromFile().then((settings) => {
      setSettings({
        interval: settings.interval_sec,
        dateLine: '12:00',
        notificationMode: 'onSuccess',
      });
    });
  }, []);

  return {
    settings,
    isValidValue,
    onChangeInterval,
    onChangeDateLine,
    onChangeNotificationMode,
    saveSettings,
  } as const;
}
