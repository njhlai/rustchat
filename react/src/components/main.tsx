"use client";

import { CSSProperties, useEffect } from "react";
import { Provider } from "react-redux";

import { leave } from "../api/user/actions";
import store from "../app/store";
import { useAppDispatch, useAppSelector } from "../app/hooks";
import Login from "./login";
import UserListWindow from "./userlist";

const styles = {
    feed: {
        flexGrow: 6,
        margin: "auto",
        textAlign: "center",
    },
    container: {
        borderTopStyle: "solid",
        display: "flex",
        flexDirection: "row",
        height: "100%",
    },
} as Record<string, CSSProperties>;

function SwitcherWindow() {
    const user = useAppSelector((state) => state.user);
    const dispatch = useAppDispatch();

    useEffect(() => {
        function leaveServer(_: BeforeUnloadEvent) {
            dispatch(leave());
        }

        window.addEventListener("beforeunload", leaveServer);
    });

    return (
        <div style={styles.feed}>{user ? "Hello " + user.name : <Login />}</div>
    );
}

export default function MainWindow() {
    return (
        <div style={styles.container}>
            <Provider store={store}>
                <UserListWindow />
                <SwitcherWindow />
            </Provider>
        </div>
    );
}
