import { ActionIcon } from '@mantine/core';
import { TimeInput } from '@mantine/dates';
import { IconClock } from '@tabler/icons-react';
import { useRef } from 'react';

type Props = {} & React.ComponentProps<typeof TimeInput>;

export default function TimeInputWithPicker(props: Props) {
  const ref = useRef<HTMLInputElement>(null);

  const pickerControl = (
    <ActionIcon variant="subtle" color="gray" onClick={() => ref.current?.showPicker()}>
      <IconClock size={16} stroke={1.5} />
    </ActionIcon>
  );

  return <TimeInput {...props} ref={ref} rightSection={pickerControl} />;
}
