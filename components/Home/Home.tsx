import Intro from '../Intro/Intro';
import Repos from '../Repos/Repos';

import styles from './Home.module.scss';

function Home(){
    return (
        <div className={styles.Home}>
            <Intro />
            <Repos />
        </div>
    )
}

export default Home;
