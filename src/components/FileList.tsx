import { FileText, X, CheckCircle, AlertCircle, Loader2, ExternalLink } from 'lucide-react'

interface ConversionFile {
  path: string;
  name: string;
  outputPath?: string;
  status: 'pending' | 'converting' | 'completed' | 'error';
  progress: number;
  error?: string;
}

interface FileListProps {
  files: ConversionFile[]
  onRemove: (index: number) => void
  onOpenPdf?: (path: string) => void
}

export default function FileList({ files, onRemove, onOpenPdf }: FileListProps) {
  const getStatusIcon = (status: string) => {
    switch (status) {
      case 'completed':
        return <CheckCircle size={20} className="text-green-500" />
      case 'error':
        return <AlertCircle size={20} className="text-red-500" />
      case 'converting':
        return <Loader2 size={20} className="text-primary-500 animate-spin" />
      default:
        return <FileText size={20} className="text-gray-400" />
    }
  }

  const getStatusText = (status: string) => {
    switch (status) {
      case 'completed':
        return 'Completed'
      case 'error':
        return 'Failed'
      case 'converting':
        return 'Converting...'
      default:
        return 'Pending'
    }
  }

  const getStatusColor = (status: string) => {
    switch (status) {
      case 'completed':
        return 'text-green-600 bg-green-50'
      case 'error':
        return 'text-red-600 bg-red-50'
      case 'converting':
        return 'text-primary-600 bg-primary-50'
      default:
        return 'text-gray-600 bg-gray-50'
    }
  }

  return (
    <div className="space-y-2 max-h-64 overflow-y-auto">
      {files.map((file, index) => (
        <div
          key={`${file.path}-${index}`}
          className="flex items-center p-3 bg-gray-50 rounded-lg hover:bg-gray-100 transition-colors"
        >
          {/* Icon */}
          <div className="flex-shrink-0 mr-3">
            {getStatusIcon(file.status)}
          </div>

          {/* File Info */}
          <div className="flex-1 min-w-0 mr-4">
            <p className="text-sm font-medium text-gray-800 truncate" title={file.name}>
              {file.name}
            </p>
            <div className="flex items-center mt-1">
              <span className={`text-xs px-2 py-0.5 rounded-full ${getStatusColor(file.status)}`}>
                {getStatusText(file.status)}
              </span>
              {file.status === 'converting' && (
                <span className="text-xs text-gray-500 ml-2">
                  {file.progress}%
                </span>
              )}
            </div>
            {file.error && (
              <p className="text-xs text-red-500 mt-1 truncate" title={file.error}>
                {file.error}
              </p>
            )}
          </div>

          {/* Progress Bar for converting files */}
          {file.status === 'converting' && (
            <div className="w-24 mr-3">
              <div className="h-1.5 bg-gray-200 rounded-full overflow-hidden">
                <div
                  className="h-full bg-primary-500 transition-all duration-300"
                  style={{ width: `${file.progress}%` }}
                />
              </div>
            </div>
          )}

          {/* Open PDF Button for completed files - PROMINENT VERSION */}
          {file.status === 'completed' && file.outputPath && onOpenPdf && (
            <button
              onClick={() => onOpenPdf(file.outputPath!)}
              className="flex-shrink-0 mr-2 px-3 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-lg font-semibold text-sm flex items-center gap-2 shadow-md hover:shadow-lg transform hover:scale-105 transition-all duration-200"
              title="Open PDF"
            >
              <ExternalLink size={18} />
              Open PDF
            </button>
          )}

          {/* Remove Button */}
          <button
            onClick={() => onRemove(index)}
            disabled={file.status === 'converting'}
            className={`flex-shrink-0 p-1 rounded transition-colors ${
              file.status === 'converting'
                ? 'opacity-50 cursor-not-allowed'
                : 'hover:bg-gray-200 text-gray-400 hover:text-gray-600'
            }`}
            title="Remove file"
          >
            <X size={18} />
          </button>
        </div>
      ))}
    </div>
  )
}
