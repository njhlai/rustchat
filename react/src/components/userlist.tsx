"use client";

import { CSSProperties } from "react";

import { useAppSelector } from "../app/hooks";

const styles = {
    userlist: {
        borderRightStyle: "solid",
        flexGrow: 1,
        marginTop: 0,
    },
} as Record<string, CSSProperties>;

export default function UserListWindow() {
    const users = useAppSelector((state) => state.feed.users);

    return (
        <>
            <div style={styles.userlist}>
                <h3>Users</h3>
                <ul>
                    {users.map((user) => (
                        <li key={user.id.toString()}>{user.name}</li>
                    ))}
                </ul>
            </div>
        </>
    );
}
