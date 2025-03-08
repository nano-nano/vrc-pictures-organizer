import { Button, Flex, NumberInput, Select, Stack, Text, Textarea } from '@mantine/core';

import { IconDeviceFloppy } from '@tabler/icons-react';
import useSettings from '../hooks/useSettings';
import TimeInputWithPicker from './TimeInputWithPicker';

export default function AppSettingsPage() {
  const { saveSettings } = useSettings();

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
              value={60}
              onChange={() => {
                /** TODO */
              }}
              disabled={false}
              style={{ width: '100%' }}
            />
            <TimeInputWithPicker
              size="xs"
              width="100%"
              label="日付の境（24時間制）"
              value={'12:00'}
              onChange={() => {
                /** TODO */
              }}
              disabled={false}
              style={{ width: '100%' }}
            />
          </Flex>
          <Select
            size="xs"
            label="処理結果通知"
            data={[
              { value: 'always', label: '常に通知する' },
              { value: 'onSuccess', label: '成功時のみ通知する' },
              { value: 'none', label: '通知しない' },
            ]}
            value={'onSuccess'}
          />
          <Button
            size="xs"
            leftSection={<IconDeviceFloppy size={20} />}
            onClick={() => {
              /** TODO */
              saveSettings();
            }}
            disabled={false}
          >
            保存
          </Button>
        </Stack>
        <Textarea size="xs" maxRows={4} label="ログ" styles={{ input: { height: '95px' } }} />
      </Stack>
    </main>
  );
}
