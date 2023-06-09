export default function copyToClipboard (data: string) {
    navigator.clipboard.writeText(data);
};