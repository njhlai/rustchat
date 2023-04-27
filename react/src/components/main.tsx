"use client";

import { Provider } from "react-redux";

import store from "../app/store";
import { serverUrl } from "../app/hooks";
import UserListWindow from "./userlist";
import SwitcherWindow from "./switcher";

import styles from "../styles/layout.module.css";

export default function MainWindow({ title }: { title?: string }) {
    return (
        <div className={styles.main}>
            <h1>
                {title ?? "rustchat"}@{serverUrl}
            </h1>
            <div className={styles.container}>
                <Provider store={store}>
                    <UserListWindow />
                    <SwitcherWindow />
                </Provider>
            </div>
        </div>
    );
}
