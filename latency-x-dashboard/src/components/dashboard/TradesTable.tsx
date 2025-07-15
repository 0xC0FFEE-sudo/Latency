import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
} from "@/components/ui/card";
import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from "@/components/ui/table";
import { Badge } from "@/components/ui/badge";
import type { Trade } from '@/hooks/useWebSocket';

export function TradesTable({ trades }: { trades: Trade[] }) {
    return (
        <Card className="col-span-3">
            <CardHeader>
                <CardTitle>Recent Trades</CardTitle>
                <CardDescription>
                    You made {trades.length} trades this month.
                </CardDescription>
            </CardHeader>
            <CardContent>
                <Table>
                    <TableHeader>
                        <TableRow>
                            <TableHead>Side</TableHead>
                            <TableHead>Symbol</TableHead>
                            <TableHead>Quantity</TableHead>
                            <TableHead>Price</TableHead>
                            <TableHead>Time</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        {trades.map((trade) => (
                            <TableRow key={trade.order_id}>
                                <TableCell>
                                    <Badge variant={trade.side === 'Buy' ? 'default' : 'destructive'}>
                                        {trade.side}
                                    </Badge>
                                </TableCell>
                                <TableCell>{trade.symbol}</TableCell>
                                <TableCell>{trade.quantity}</TableCell>
                                <TableCell>{trade.price.toFixed(2)}</TableCell>
                                <TableCell>{new Date(trade.timestamp).toLocaleTimeString()}</TableCell>
                            </TableRow>
                        ))}
                    </TableBody>
                </Table>
            </CardContent>
        </Card>
    )
} 