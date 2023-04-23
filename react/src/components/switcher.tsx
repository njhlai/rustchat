"use client";

import { CSSProperties, useEffect } from "react";

import { leave } from "../api/user/actions";
import { useAppDispatch, useAppSelector } from "../app/hooks";
import Login from "./login";
import Feed from "./feed";

const styles = {
    feed: {
        display: "flex",
        flexDirection: "column",
        flexGrow: 5,
        margin: "auto",
        textAlign: "center",
    },
} as Record<string, CSSProperties>;

export default function SwitcherWindow() {
    const user = useAppSelector((state) => state.user);
    const dispatch = useAppDispatch();

    useEffect(() => {
        function leaveServer(_: BeforeUnloadEvent) {
            dispatch(leave());
        }

        window.addEventListener("beforeunload", leaveServer);
    });

    return (
        <div style={styles.feed}>
            {user ? <Feed username={user.name} /> : <Login />}
        </div>
    );
}
