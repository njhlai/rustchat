import MainWindow from "../components/main";

export const metadata = {
    title: "rustchat",
    description: "React frontend for rustchat",
};

const serverUrl = "localhost:8080";

export default function Page() {
    return (
        <>
            <h1>
                {metadata.title}@{serverUrl}
            </h1>
            <MainWindow />
        </>
    );
}
