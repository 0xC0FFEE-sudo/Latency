import { ResponsiveContainer, LineChart, XAxis, YAxis, Tooltip, Line } from 'recharts';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import type { LatencyUpdate } from '@/hooks/useWebSocket';

export function LatencyChart({ data }: { data: LatencyUpdate[] }) {
    return (
        <Card className="col-span-4">
            <CardHeader>
                <CardTitle>Tick-to-Trade Latency (µs)</CardTitle>
            </CardHeader>
            <CardContent className="pl-2">
                <ResponsiveContainer width="100%" height={350}>
                    <LineChart data={data.slice().reverse()}>
                        <XAxis dataKey="order_id" tick={{ fill: 'hsl(var(--muted-foreground))', fontSize: 12 }} />
                        <YAxis tick={{ fill: 'hsl(var(--muted-foreground))', fontSize: 12 }} />
                        <Tooltip
                            contentStyle={{
                                backgroundColor: 'hsl(var(--background))',
                                border: '1px solid hsl(var(--border))',
                            }}
                        />
                        <Line type="monotone" dataKey="latency_us" stroke="hsl(var(--primary))" strokeWidth={2} dot={false} />
                    </LineChart>
                </ResponsiveContainer>
            </CardContent>
        </Card>
    );
} 