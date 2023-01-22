
declare global {
  interface Window {
    xnft: any;
  }
}
import { WalletModalProvider } from '@solana/wallet-adapter-react-ui'
import { WalletProvider, ConnectionProvider} from '@solana/wallet-adapter-react'
import { css } from '@emotion/react';
import { GlobalStyles } from '@mui/material';
import React from 'react';
import NavigationBar from './components/NavigationBar';
import DataLayer from './data';
import Router from './Router';
import { zIndices } from './util/const';
import { PhantomWalletAdapter } from '@solana/wallet-adapter-phantom';

const inputGlobalStyles = (
  <GlobalStyles
    styles={css`
      [data-reach-dialog-overlay] {
        z-index: ${zIndices.ConnectWalletDialog};
      }
    `}
  />
);

const App: React.FC = () => {
  document.body.style.backgroundColor = '#1c1c1c';

  return (
    <ConnectionProvider endpoint='https://rpc.helius.xyz/?api-key=6b1ccd35-ba2d-472a-8f54-9ac2c3c40b8b'>
    <WalletProvider wallets={[window.xnft.solana ?? new PhantomWalletAdapter()]} >
      <WalletModalProvider>      <DataLayer>
        {inputGlobalStyles}
        <div style={{ display: 'flex', flexDirection: 'column' }}>
          <NavigationBar />
          <Router />
        </div>
      </DataLayer></WalletModalProvider></WalletProvider>
      </ConnectionProvider>

        );
};

export default App;
