"use client";

import { CSSProperties } from "react";
import { Provider } from "react-redux";

import store from "../app/store";
import { serverUrl } from "../app/hooks";
import UserListWindow from "./userlist";
import SwitcherWindow from "./switcher";

const styles = {
    container: {
        borderTopStyle: "solid",
        display: "flex",
        flexDirection: "row",
        height: "100%",
        justifyContent: "space-between",
    },
} as Record<string, CSSProperties>;

export default function MainWindow({ title }: { title?: string }) {
    return (
        <>
            <h1>
                {title ?? "rustchat"}@{serverUrl}
            </h1>
            <div style={styles.container}>
                <Provider store={store}>
                    <UserListWindow />
                    <SwitcherWindow />
                </Provider>
            </div>
        </>
    );
}
