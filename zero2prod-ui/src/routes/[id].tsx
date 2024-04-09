import { useParams } from "@solidjs/router";

export default function UserId() {
    const params = useParams();
    return (
        <div>
            <h1 class="text-3xl font-bold">Hello, {params.id}</h1>
        </div>
    )
}