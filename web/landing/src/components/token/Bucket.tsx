export interface BucketProps {
    fill: number;
}

export default function Bucket (props: BucketProps) {
    return (
        <div class="relative rounded-sm overflow-clip">
            <div class="absolute left-0 right-0 bottom-0 rounded-sm bucket-gradient"
                style={{top: ((1-Math.min(1, Math.max(0, props.fill)))*100).toString()+"%"}}
            />
        </div>
    );
}