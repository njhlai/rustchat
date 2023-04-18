"use client";

import { CSSProperties, ChangeEvent, FormEvent, useState } from "react";

const styles = {
    joinButton: {
        marginLeft: 10,
        marginRight: 10,
    },
    login: {
        display: "flex",
        flexDirection: "column",
        flexGrow: 6,
        margin: "auto",
        textAlign: "center",
    },
} as Record<string, CSSProperties>;

export default function Login() {
    const [name, setName] = useState("");

    function handleNameChange(event: ChangeEvent<HTMLInputElement>) {
        setName(event.target.value);
    }

    function handleJoin(event: FormEvent) {
        event.preventDefault();
        console.log(name);
    }

    return (
        <>
            <div style={styles.login}>
                <h3>Welcome!</h3>
                <input
                    type="text"
                    name="username"
                    value={name}
                    onChange={handleNameChange}
                />
                <button style={styles.joinButton} onClick={handleJoin}>
                    Join!
                </button>
            </div>
        </>
    );
}
