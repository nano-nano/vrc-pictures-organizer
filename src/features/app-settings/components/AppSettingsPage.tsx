import { Button, Flex, NumberInput, Select, Stack, Text, Textarea } from '@mantine/core';

import { IconDeviceFloppy } from '@tabler/icons-react';
import { useEffect } from 'react';
import { NotificationMode } from '../constants/notificationMode';
import useAppLog from '../hooks/useAppLog';
import useSettings from '../hooks/useSettings';
import TimeInputWithPicker from './TimeInputWithPicker';

export default function AppSettingsPage() {
  const {
    settings,
    isValidValue,
    onChangeInterval,
    onChangeDateLine,
    onChangeNotificationMode,
    saveSettings,
  } = useSettings();
  const { formattedLogs, fetchAppLog } = useAppLog();

  useEffect(() => {
    fetchAppLog();
  }, [fetchAppLog]);

  return (
    <main>
      <Stack px="8px" gap="12px">
        <Stack gap="8px">
          <Flex direction="row" gap="4px">
            <NumberInput
              size="xs"
              label="監視間隔（10～3600秒）"
              rightSection={<Text fz="sm">秒</Text>}
              max={3600}
              min={10}
              value={settings.interval}
              onChange={(val) => {
                onChangeInterval(Number(val));
              }}
              disabled={false}
              style={{ width: '100%' }}
            />
            <TimeInputWithPicker
              size="xs"
              width="100%"
              label="日付の境（24時間制）"
              value={settings.dateLine}
              onChange={(event) => {
                onChangeDateLine(event.target.value);
              }}
              disabled={false}
              style={{ width: '100%' }}
            />
          </Flex>
          <Select
            size="xs"
            label="処理結果通知"
            data={NotificationMode}
            value={settings.notificationMode}
            onChange={(newVal) => {
              onChangeNotificationMode(newVal);
            }}
          />
          <Button
            size="xs"
            leftSection={<IconDeviceFloppy size={20} />}
            onClick={saveSettings}
            disabled={!isValidValue}
          >
            保存
          </Button>
        </Stack>
        <Textarea
          size="xs"
          maxRows={4}
          label="ログ"
          styles={{ input: { height: '95px' } }}
          value={formattedLogs}
        />
      </Stack>
    </main>
  );
}
