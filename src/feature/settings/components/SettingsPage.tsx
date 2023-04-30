import {
  Text,
  Stack,
  TextInput,
  Textarea,
  Button,
  Flex,
  NumberInput,
} from '@mantine/core';
import SaveIcon from '@mui/icons-material/Save';

import useSetting from '../hooks/useSettings';

function SettingsPage() {
  const {
    interval,
    onChangeInterval,
    dateLineTime,
    onChangeDateLineTime,
    isErrorDateLineTime,
    onClickSave,
    isSaving,
  } = useSetting();

  return (
    <Stack sx={{ padding: '8px' }}>
      <Stack spacing="xs">
        <Flex direction="row" gap="sm">
          <NumberInput
            size="xs"
            label="フォルダ監視間隔（10～3600秒）"
            rightSection={<Text fz="sm">秒</Text>}
            max={3600}
            min={10}
            value={interval}
            onChange={onChangeInterval}
            disabled={isSaving}
            sx={{ width: '100%' }}
          />
          <TextInput
            type="time"
            size="xs"
            label="日付の境とする時刻（24時間制）"
            value={dateLineTime}
            onChange={(e) => onChangeDateLineTime(e.target.value)}
            error={isErrorDateLineTime}
            disabled={isSaving}
            sx={{ width: '100%' }}
          />
        </Flex>
      </Stack>
      <Button
        size="xs"
        leftIcon={<SaveIcon fontSize="small" />}
        onClick={onClickSave}
        disabled={isErrorDateLineTime || isSaving}
      >
        保存
      </Button>
      <Textarea size="xs" label="ログ" minRows={6} readOnly />
    </Stack>
  );
}

export default SettingsPage;
