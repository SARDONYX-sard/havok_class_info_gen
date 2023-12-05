import ClearAllIcon from '@mui/icons-material/ClearAll';
import { Box, Button, FormGroup, TextField } from '@mui/material';
import Grid from '@mui/material/Unstable_Grid2';
import { Controller, useForm, type SubmitHandler } from 'react-hook-form';
import { toast } from 'react-hot-toast';

import { ExecuteButton, LogFileButton } from '@/components/buttons';
import { SelectLogLevel } from '@/components/lists';
import { useTranslation } from '@/hooks';
import { LogLevel, add_num } from '@/tauri_cmd';
import { selectLogLevel } from '@/utils/selector';

type FormProps = {
  num1: string;
  num2: string;
  loading: boolean;
  logLevel: LogLevel;
};

const getInitialFormValues = (): FormProps => ({
  num1: localStorage.getItem('num1') ?? '',
  num2: localStorage.getItem('num2') ?? '',
  loading: false as boolean,
  logLevel: selectLogLevel(localStorage.getItem('logLevel') ?? 'error'),
});

export function ExecuteForm() {
  const { t } = useTranslation();
  const { register, handleSubmit, control, setValue } = useForm({
    mode: 'onBlur',
    criteriaMode: 'all',
    shouldFocusError: false,
    defaultValues: getInitialFormValues(),
  });

  const setStorage = (key: keyof FormProps) => {
    return (value: string) => {
      localStorage.setItem(key, value);
      setValue(key, value);
    };
  };

  const setLoading = (loading: boolean) => setValue('loading', loading);
  const onSubmit: SubmitHandler<FormProps> = async ({ num1, num2 }) => {
    setLoading(true);

    try {
      const completeInfo = await add_num({ num1, num2 });
      toast.success(completeInfo);
    } catch (err) {
      toast.error(`${err}`);
    } finally {
      setLoading(false);
    }
  };

  return (
    <Grid sx={{ display: 'block', width: '95vw' }} container component="form" onSubmit={handleSubmit(onSubmit)}>
      <Button
        sx={{ width: '100%', marginBottom: '15px' }}
        onClick={() => (['num1', 'num2'] as const).forEach((key) => setStorage(key)(''))}
        startIcon={<ClearAllIcon />}
        variant="outlined"
      >
        {t('all-clear-btn')}
      </Button>
      <FormGroup onSubmit={handleSubmit(onSubmit)}>
        <Controller
          name="num1"
          control={control}
          rules={{
            required: 'Need Number',
          }}
          render={({ field: { onChange, onBlur, value }, fieldState: { error } }) => (
            <Grid container spacing={2}>
              <Grid xs={10}>
                <TextField
                  sx={{ width: '100%' }}
                  label={t('num1')}
                  placeholder="1"
                  required
                  value={value}
                  variant="outlined"
                  margin="dense"
                  onChange={(e) => {
                    localStorage.setItem('num1', e.target.value);
                    onChange(e);
                  }}
                  onBlur={onBlur}
                  error={Boolean(error)}
                  helperText={<>{t('num1-helper')}</>}
                />
              </Grid>
            </Grid>
          )}
        />

        <Controller
          name="num2"
          control={control}
          rules={{
            required: 'Need Number',
          }}
          render={({ field: { onChange, onBlur, value }, fieldState: { error } }) => (
            <Grid container spacing={2}>
              <Grid xs={10}>
                <TextField
                  sx={{ width: '100%' }}
                  label={t('num2')}
                  placeholder="1"
                  required
                  value={value}
                  variant="outlined"
                  margin="dense"
                  onChange={(e) => {
                    localStorage.setItem('num2', e.target.value);
                    onChange(e);
                  }}
                  onBlur={onBlur}
                  error={Boolean(error)}
                  helperText={<>{t('num2-helper')}</>}
                />
              </Grid>
            </Grid>
          )}
        />

        <Grid container spacing={2}>
          <Grid xs={3}>
            <Controller
              name="logLevel"
              control={control}
              render={({ field: { value } }) => <SelectLogLevel value={value} {...register('logLevel')} />}
            />
          </Grid>

          <Grid xs={3}>
            <LogFileButton />
          </Grid>
        </Grid>

        <Controller
          name="loading"
          control={control}
          render={({ field: { value } }) => (
            <Box sx={{ width: '100%', paddingTop: '10px' }}>
              <ExecuteButton loading={value} setLoading={setLoading} />
            </Box>
          )}
        />
      </FormGroup>
    </Grid>
  );
}
