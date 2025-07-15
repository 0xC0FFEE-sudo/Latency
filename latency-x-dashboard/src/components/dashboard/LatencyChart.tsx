import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
} from '@/components/ui/card';
import { ResponsiveContainer, LineChart, Line, XAxis, YAxis, Tooltip } from 'recharts';
import type { LatencyUpdate } from '@/hooks/useWebSocket';

export function LatencyChart({ data }: { data: LatencyUpdate[] }) {
    return (
        <Card className="col-span-4">
            <CardHeader>
                <CardTitle>Latency</CardTitle>
                <CardDescription>
                    Tick-to-trade latency over the last 24 hours.
                </CardDescription>
            </CardHeader>
            <CardContent className="pl-2">
                <ResponsiveContainer width="100%" height={350}>
                    <LineChart data={data}>
                        <XAxis dataKey="order_id" stroke="#888888" fontSize={12} tickLine={false} axisLine={false} />
                        <YAxis stroke="#888888" fontSize={12} tickLine={false} axisLine={false} tickFormatter={(value) => `${value}µs`} />
                        <Tooltip />
                        <Line type="monotone" dataKey="latency_us" stroke="#8884d8" activeDot={{ r: 8 }} />
                    </LineChart>
                </ResponsiveContainer>
            </CardContent>
        </Card>
    )
} 