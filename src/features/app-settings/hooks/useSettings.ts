import { notifications } from '@mantine/notifications';

export default function useSettings() {
  function saveSettings() {
    // TODO
    notifications.show({
      title: '保存成功',
      message: '設定を保存しました',
      withBorder: true,
      withCloseButton: false,
    });
  }

  return { saveSettings } as const;
}
