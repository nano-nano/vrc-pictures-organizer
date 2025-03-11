export type NotificationModeValue = 'always' | 'onSuccess' | 'none';

export const NotificationMode: { value: NotificationModeValue; label: string }[] = [
  { value: 'always', label: '常に通知する' },
  { value: 'onSuccess', label: '成功時のみ通知する' },
  { value: 'none', label: '通知しない' },
];

export const NOTIFICATION_MODE_VALUES = NotificationMode.map((e) => e.value) as string[];
