import { AppProps } from 'next/app';
import HeadData from '../components/HeadData/HeadData';
import Header from '../components/Header/Header';
import '../styles/index.scss';
import styles from './App.module.scss';

function App({ Component, pageProps }: AppProps) {
    return (
        <>
            <HeadData /><div className={styles.App}>
                <Header />
                <div className={styles.bodyContainer}>
                    <Component {...pageProps} />
                </div>
            </div>
        </>
    );

}

export default App;
