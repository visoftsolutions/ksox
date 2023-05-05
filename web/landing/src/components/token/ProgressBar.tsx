export interface ProgressBarProps {
    fill: number;
}

export default function ProgressBar (props: ProgressBarProps) {
    return (
        <div class="relative rounded-sm overflow-clip h-8 stripes">
            <div class="absolute left-0 top-0 bottom-0 rounded-sm token-linear-wipe-button"
                style={{right: ((1-Math.min(1, Math.max(0, props.fill)))*100).toString()+"%"}}
            />
        </div>
    );
}