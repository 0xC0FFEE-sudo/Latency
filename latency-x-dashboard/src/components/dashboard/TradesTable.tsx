import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from "@/components/ui/table";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import type { Trade } from "@/hooks/useWebSocket";
import { Badge } from "@/components/ui/badge";

export function TradesTable({ trades }: { trades: Trade[] }) {
    return (
        <Card>
            <CardHeader>
                <CardTitle>Recent Trades</CardTitle>
            </CardHeader>
            <CardContent>
                <Table>
                    <TableHeader>
                        <TableRow>
                            <TableHead>Symbol</TableHead>
                            <TableHead>Side</TableHead>
                            <TableHead className="text-right">Price</TableHead>
                            <TableHead className="text-right">Quantity</TableHead>
                            <TableHead className="text-right">Time</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        {trades.map((trade) => (
                            <TableRow key={trade.order_id}>
                                <TableCell>{trade.symbol}</TableCell>
                                <TableCell>
                                    <Badge variant={trade.side === 'Buy' ? 'default' : 'destructive'}>
                                        {trade.side}
                                    </Badge>
                                </TableCell>
                                <TableCell className="text-right">{trade.price.toFixed(2)}</TableCell>
                                <TableCell className="text-right">{trade.quantity}</TableCell>
                                <TableCell className="text-right">{new Date(trade.timestamp).toLocaleTimeString()}</TableCell>
                            </TableRow>
                        ))}
                    </TableBody>
                </Table>
            </CardContent>
        </Card>
    );
} 