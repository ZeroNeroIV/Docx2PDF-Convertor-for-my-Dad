import { useState, useCallback } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { Upload, FileText } from 'lucide-react'

interface DropZoneProps {
  onFilesAdded: (files: string[]) => void
  disabled?: boolean
}

export default function DropZone({ onFilesAdded, disabled = false }: DropZoneProps) {
  const [isDragging, setIsDragging] = useState(false)

  const handleDragEnter = useCallback((e: React.DragEvent) => {
    e.preventDefault()
    e.stopPropagation()
    if (!disabled) {
      setIsDragging(true)
    }
  }, [disabled])

  const handleDragLeave = useCallback((e: React.DragEvent) => {
    e.preventDefault()
    e.stopPropagation()
    setIsDragging(false)
  }, [])

  const handleDragOver = useCallback((e: React.DragEvent) => {
    e.preventDefault()
    e.stopPropagation()
  }, [])

  const handleDrop = useCallback(async (e: React.DragEvent) => {
    e.preventDefault()
    e.stopPropagation()
    setIsDragging(false)

    if (disabled) return

    const files: string[] = []
    
    // Handle dropped files
    if (e.dataTransfer.files) {
      for (let i = 0; i < e.dataTransfer.files.length; i++) {
        const file = e.dataTransfer.files[i]
        if (file.name.toLowerCase().endsWith('.docx')) {
          // Get the file path using Tauri
          try {
            const path = await invoke<string>('get_dropped_file_path', { 
              name: file.name,
              size: file.size
            })
            if (path) {
              files.push(path)
            }
          } catch (error) {
            console.error('Failed to get file path:', error)
          }
        }
      }
    }

    if (files.length > 0) {
      onFilesAdded(files)
    }
  }, [onFilesAdded, disabled])

  const handleFileSelect = useCallback(async () => {
    if (disabled) return

    try {
      const selectedFiles = await invoke<string[]>('select_files')
      if (selectedFiles && selectedFiles.length > 0) {
        onFilesAdded(selectedFiles)
      }
    } catch (error) {
      console.error('Failed to select files:', error)
    }
  }, [onFilesAdded, disabled])

  return (
    <div
      onDragEnter={handleDragEnter}
      onDragLeave={handleDragLeave}
      onDragOver={handleDragOver}
      onDrop={handleDrop}
      className={`drop-zone ${isDragging ? 'dragging' : ''} ${disabled ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer'}`}
      onClick={!disabled ? handleFileSelect : undefined}
    >
      <div className="space-y-4">
        <div className="flex justify-center">
          <div className={`p-4 rounded-full transition-all duration-200 ${
            isDragging ? 'bg-primary-100 scale-110' : 'bg-gray-100'
          }`}>
            {isDragging ? (
              <FileText size={48} className="text-primary-600" />
            ) : (
              <Upload size={48} className="text-gray-400" />
            )}
          </div>
        </div>
        
        <div>
          <p className="text-lg font-medium text-gray-700">
            {isDragging ? 'Drop files here' : 'Drag & drop DOCX files here'}
          </p>
          <p className="text-sm text-gray-500 mt-1">
            or click to browse
          </p>
        </div>
        
        <div className="flex justify-center space-x-2 text-xs text-gray-400">
          <span className="bg-gray-100 px-2 py-1 rounded">.docx</span>
          <span>files only</span>
        </div>
      </div>
    </div>
  )
}
