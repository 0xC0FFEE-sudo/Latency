import { useWebSocket } from "@/hooks/useWebSocket";
import { Badge } from "@/components/ui/badge";

export function LogsPage() {
    const { logs } = useWebSocket('ws://localhost:3000/ws');

    const getBadgeVariant = (level: string) => {
        switch (level) {
            case 'ERROR':
                return 'destructive';
            case 'WARN':
                return 'secondary';
            case 'INFO':
                return 'default';
            case 'DEBUG':
                return 'outline';
            case 'TRACE':
                return 'outline';
            default:
                return 'default';
        }
    }

    return (
        <div className="w-full">
            <h1 className="text-2xl font-bold mb-4">Live Logs</h1>
            <div className="bg-muted p-4 rounded-md font-mono text-xs overflow-auto h-[70vh]">
                {logs.map((log, index) => (
                    <div key={index} className="flex items-center space-x-4 py-1 border-b border-muted-foreground/20">
                        <span className="text-muted-foreground">{new Date(log.timestamp).toLocaleTimeString()}</span>
                        <Badge variant={getBadgeVariant(log.level)} className="w-16 flex-shrink-0 justify-center">{log.level}</Badge>
                        <span className="text-muted-foreground/80 w-48 flex-shrink-0 truncate">{log.target}</span>
                        <span>{log.message}</span>
                    </div>
                ))}
            </div>
        </div>
    )
} 