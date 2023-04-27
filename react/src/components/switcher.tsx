"use client";

import { useEffect } from "react";

import { leave } from "../api/user/actions";
import { useAppDispatch, useAppSelector } from "../app/hooks";
import Login from "./login";
import Feed from "./feed";

export default function SwitcherWindow() {
    const user = useAppSelector((state) => state.user);
    const dispatch = useAppDispatch();

    useEffect(() => {
        function leaveServer(_: BeforeUnloadEvent) {
            dispatch(leave());
        }

        window.addEventListener("beforeunload", leaveServer);
    });

    return user ? <Feed username={user.name} /> : <Login />;
}
