import ConvertIcon from '@mui/icons-material/Transform';
import LoadingButton from '@mui/lab/LoadingButton';

import { useTranslation } from '@/hooks';

type Props = Readonly<{
  loading: boolean;
  setLoading: (loading: boolean) => void;
}>;

/**
 *
 * Icon ref
 * - https://mui.com/material-ui/material-icons/
 */
export function ExecuteButton({ loading, setLoading }: Props) {
  const { t } = useTranslation();

  return (
    <LoadingButton
      type="submit"
      sx={{ width: '100%' }}
      endIcon={<ConvertIcon />}
      loading={loading}
      loadingPosition="end"
      variant="contained"
      onChange={() => setLoading(true)}
    >
      <span>{loading ? t('executing-btn') : t('execute-btn')}</span>
    </LoadingButton>
  );
}
