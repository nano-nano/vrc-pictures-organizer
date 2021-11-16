import * as React from 'react';
import { useEffect, useState } from 'react';
import * as ReactDOM from 'react-dom';

import {
  Box,
  Button,
  ChakraProvider,
  FormControl,
  FormLabel,
  Input,
  InputGroup,
  InputRightElement,
  useToast,
  VStack
} from '@chakra-ui/react'
import { IpcApi } from './preload';

const App = () => {
  const ipcApi = window.ipcApi as IpcApi;

  const toast = useToast();
  const [ folderPath, setFolderPath ] = useState('');
  const [ isRunOrganize, setIsRunOrganize ] = useState(false);

  useEffect(() => {
    ipcApi.registerOnFinishFileOrganizeCallback((successCount, failCount) => {
      setIsRunOrganize(false);
      toast({
        title: '処理終了',
        description: (failCount > 0 ? `処理に失敗したファイルが${failCount}件あります` : `${successCount}件処理しました`),
        status: (failCount > 0 ? 'error' : 'success'),
        duration: 3000,
        isClosable: true,
      });
    })
    ipcApi.getFolderPath().then(result => setFolderPath(result));
  }, []);

  const onClickSavePathButton = () => ipcApi.saveSettingsFile(getSettingsObject(), (isSucceeded) => {
    toast({
      description: (isSucceeded ? '保存しました' : '保存に失敗しました'),
      status: (isSucceeded ? 'success' : 'error'),
      duration: 3000,
      isClosable: true,
    });
  });
  const onClickExecButton = () => {
    setIsRunOrganize(true);
    ipcApi.execFileOrganize(folderPath);
  };

  const getSettingsObject = () => {
    return {
      folderPath: folderPath
    };
  }

  return (
    <>
      <Box
        bg='gray.100'
        w='100vw'
        h='100vh'
        p='2'
      >
        <VStack spacing='4'>
          <Box w='100%'>
            <FormControl id='folder-path'>
              <FormLabel>画像が保存されているフォルダパス</FormLabel>
              <InputGroup size="md">
                <Input
                  type='text'
                  bg='white'
                  pr='4.5rem'
                  value={folderPath}
                  disabled={isRunOrganize}
                  onChange={(event) => setFolderPath(event.target.value)}
                />
                <InputRightElement width='4.5rem'>
                  <Button
                    h='1.75rem'
                    size='sm'
                    disabled={isRunOrganize}
                    onClick={onClickSavePathButton}
                  >
                    保存
                  </Button>
                </InputRightElement>
              </InputGroup>
            </FormControl>
          </Box>
          <Button
            colorScheme='teal'
            variant='solid'
            isFullWidth
            isLoading={isRunOrganize}
            onClick={onClickExecButton}
          >
            振り分け実行
          </Button>
        </VStack>
      </Box>
    </>
  );
};

ReactDOM.render(
  <React.StrictMode>
    <ChakraProvider>
      <App />
    </ChakraProvider>
  </React.StrictMode>,
  document.getElementById('root')
);