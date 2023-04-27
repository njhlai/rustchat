"use client";

import { useAppSelector } from "../app/hooks";

import styles from "../styles/layout.module.css";

export default function UserListWindow() {
    const myself = useAppSelector((state) => state.user);
    const users = useAppSelector((state) => state.feed.users);

    return (
        <div className={styles.users}>
            {myself ? (
                <p>
                    <strong>Myself:</strong>{" "}
                    <span className={styles.myself}>{myself.name}</span> @{" "}
                    <span className={styles.myid}>{myself.id.toString()}</span>
                </p>
            ) : (
                <></>
            )}
            <p>
                <b>Users</b>
            </p>
            <ul>
                {users.map((user) => {
                    if (user.id != myself?.id) {
                        return (
                            <li key={user.id.toString()}>
                                <span className={styles.name}>{user.name}</span>{" "}
                                @{" "}
                                <span className={styles.id}>
                                    {user.id.toString()}
                                </span>
                            </li>
                        );
                    }
                })}
            </ul>
        </div>
    );
}
