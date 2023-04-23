"use client";

import { CSSProperties } from "react";

import { useAppSelector } from "../app/hooks";

const styles = {
    id: {
        color: "yellow",
    },
    name: {
        color: "cyan",
    },
    myid: {
        color: "orange",
    },
    myself: {
        color: "green",
    },
    users: {
        borderRightStyle: "solid",
        flexGrow: 1,
        marginTop: 0,
    },
    userlist: {
        paddingLeft: 10,
    },
} as Record<string, CSSProperties>;

export default function UserListWindow() {
    const myself = useAppSelector((state) => state.user);
    const users = useAppSelector((state) => state.feed.users);

    return (
        <>
            <div style={styles.users}>
                {myself ? (
                    <p>
                        <strong>Myself:</strong>{" "}
                        <span style={styles.myself}>{myself.name}</span> @{" "}
                        <span style={styles.myid}>{myself.id.toString()}</span>
                    </p>
                ) : (
                    <></>
                )}
                <h3>Users</h3>
                <ul style={styles.userlist}>
                    {users.map((user) => {
                        if (user.id != myself?.id) {
                            return (
                                <li key={user.id.toString()}>
                                    <span style={styles.name}>{user.name}</span>{" "}
                                    @{" "}
                                    <span style={styles.id}>
                                        {user.id.toString()}
                                    </span>
                                </li>
                            );
                        }
                    })}
                </ul>
            </div>
        </>
    );
}
