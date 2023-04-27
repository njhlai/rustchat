import MainWindow from "../components/main";

export const metadata = {
    title: "rustchat",
    description: "React frontend for rustchat",
};

export default function Page() {
    return (
        <>
            <MainWindow title={metadata.title} />
        </>
    );
}
