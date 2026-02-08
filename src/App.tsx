import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
import DropZone from './components/DropZone'
import FileList from './components/FileList'
import ProgressBar from './components/ProgressBar'
import { FileText, CheckCircle, AlertCircle, FolderOpen } from 'lucide-react'

interface ConversionFile {
  path: string;
  name: string;
  status: 'pending' | 'converting' | 'completed' | 'error';
  progress: number;
  error?: string;
}

interface ConversionProgress {
  file_path: string;
  progress: number;
  status: string;
  error?: string;
}

function App() {
  const [files, setFiles] = useState<ConversionFile[]>([])
  const [outputDir, setOutputDir] = useState<string>('')
  const [isConverting, setIsConverting] = useState(false)
  const [overallProgress, setOverallProgress] = useState(0)
  const [updateAvailable, setUpdateAvailable] = useState<string | null>(null)
  const [isCheckingUpdate, setIsCheckingUpdate] = useState(false)

  useEffect(() => {
    // Listen for conversion progress updates
    const unlisten = listen<ConversionProgress>('conversion-progress', (event) => {
      const { file_path, progress, status, error } = event.payload
      
      setFiles(prev => prev.map(f => {
        if (f.path === file_path) {
          return {
            ...f,
            progress,
            status: status as 'pending' | 'converting' | 'completed' | 'error',
            error
          }
        }
        return f
      }))
    })

    // Check for updates on startup (non-blocking)
    checkForUpdates()

    return () => {
      unlisten.then(f => f())
    }
  }, [])

  useEffect(() => {
    // Calculate overall progress
    if (files.length === 0) {
      setOverallProgress(0)
    } else {
      const totalProgress = files.reduce((sum, f) => sum + f.progress, 0)
      setOverallProgress(totalProgress / files.length)
    }
  }, [files])

  const checkForUpdates = async () => {
    setIsCheckingUpdate(true)
    try {
      const version = await invoke<string>('check_for_updates').catch(() => null)
      if (version) {
        setUpdateAvailable(version)
      }
    } catch (error) {
      // Silently fail - offline is okay
    } finally {
      setIsCheckingUpdate(false)
    }
  }

  const handleFilesAdded = (newFiles: string[]) => {
    const fileObjects: ConversionFile[] = newFiles.map(path => ({
      path,
      name: path.split(/[\\/]/).pop() || path,
      status: 'pending',
      progress: 0
    }))
    
    setFiles(prev => [...prev, ...fileObjects])
  }

  const handleRemoveFile = (index: number) => {
    if (!isConverting) {
      setFiles(prev => prev.filter((_, i) => i !== index))
    }
  }

  const handleSelectOutputDir = async () => {
    try {
      const dir = await invoke<string>('select_output_directory')
      setOutputDir(dir)
    } catch (error) {
      console.error('Failed to select directory:', error)
    }
  }

  const handleConvert = async () => {
    if (files.length === 0 || isConverting) return

    setIsConverting(true)
    
    try {
      // Reset all files to pending
      setFiles(prev => prev.map(f => ({ ...f, status: 'pending', progress: 0 })))

      // Convert all files
      await invoke('convert_batch', {
        files: files.map(f => f.path),
        outputDir: outputDir || null
      })

    } catch (error) {
      console.error('Conversion failed:', error)
    } finally {
      setIsConverting(false)
    }
  }

  const handleClearCompleted = () => {
    if (!isConverting) {
      setFiles(prev => prev.filter(f => f.status !== 'completed'))
    }
  }

  const completedCount = files.filter(f => f.status === 'completed').length
  const errorCount = files.filter(f => f.status === 'error').length

  return (
    <div className="min-h-screen bg-gradient-to-br from-gray-50 to-gray-100 p-6">
      <div className="max-w-4xl mx-auto">
        {/* Header */}
        <div className="text-center mb-8">
          <h1 className="text-4xl font-bold text-gray-800 mb-2">
            <FileText className="inline-block mr-2 mb-1" size={36} />
            Docx2PDF Converter
          </h1>
          <p className="text-gray-600">
            Simple offline document conversion for your dad
          </p>
        </div>

        {/* Update Notification */}
        {updateAvailable && (
          <div className="mb-4 p-4 bg-blue-50 border border-blue-200 rounded-lg flex items-center justify-between">
            <div className="flex items-center text-blue-800">
              <AlertCircle className="mr-2" size={20} />
              <span>Version {updateAvailable} is available!</span>
            </div>
            <button className="text-blue-600 hover:text-blue-800 font-semibold">
              Download Update
            </button>
          </div>
        )}

        {/* Main Card */}
        <div className="card mb-6">
          <DropZone 
            onFilesAdded={handleFilesAdded} 
            disabled={isConverting}
          />
        </div>

        {/* File List */}
        {files.length > 0 && (
          <div className="card mb-6">
            <div className="flex items-center justify-between mb-4">
              <h2 className="text-xl font-semibold text-gray-800">
                Files ({files.length})
              </h2>
              {completedCount > 0 && (
                <button
                  onClick={handleClearCompleted}
                  disabled={isConverting}
                  className="text-sm text-gray-500 hover:text-gray-700 disabled:opacity-50"
                >
                  Clear completed
                </button>
              )}
            </div>
            
            <FileList 
              files={files} 
              onRemove={handleRemoveFile}
            />
          </div>
        )}

        {/* Output Directory */}
        {files.length > 0 && (
          <div className="card mb-6">
            <div className="flex items-center justify-between">
              <div className="flex-1 mr-4">
                <label className="block text-sm font-medium text-gray-700 mb-1">
                  Output Directory
                </label>
                <div className="text-sm text-gray-600 truncate">
                  {outputDir || 'Same as input files (click to change)'}
                </div>
              </div>
              <button
                onClick={handleSelectOutputDir}
                disabled={isConverting}
                className="btn-secondary flex items-center"
              >
                <FolderOpen className="mr-2" size={18} />
                Browse
              </button>
            </div>
          </div>
        )}

        {/* Progress Bar */}
        {isConverting && (
          <div className="card mb-6">
            <ProgressBar progress={overallProgress} />
            <p className="text-center text-sm text-gray-600 mt-2">
              Converting... {Math.round(overallProgress)}%
            </p>
          </div>
        )}

        {/* Convert Button */}
        {files.length > 0 && (
          <div className="card">
            <button
              onClick={handleConvert}
              disabled={isConverting || files.every(f => f.status === 'completed')}
              className={`w-full btn-primary flex items-center justify-center text-lg py-4 ${
                isConverting || files.every(f => f.status === 'completed')
                  ? 'opacity-50 cursor-not-allowed'
                  : ''
              }`}
            >
              {isConverting ? (
                <>
                  <div className="animate-spin rounded-full h-5 w-5 border-b-2 border-white mr-2"></div>
                  Converting...
                </>
              ) : files.every(f => f.status === 'completed') ? (
                <>
                  <CheckCircle className="mr-2" size={20} />
                  All Done!
                </>
              ) : (
                <>
                  Convert {files.filter(f => f.status !== 'completed').length} File(s) to PDF
                </>
              )}
            </button>

            {/* Status Summary */}
            {(completedCount > 0 || errorCount > 0) && (
              <div className="mt-4 flex justify-center space-x-6 text-sm">
                {completedCount > 0 && (
                  <span className="text-green-600 flex items-center">
                    <CheckCircle className="mr-1" size={16} />
                    {completedCount} completed
                  </span>
                )}
                {errorCount > 0 && (
                  <span className="text-red-600 flex items-center">
                    <AlertCircle className="mr-1" size={16} />
                    {errorCount} failed
                  </span>
                )}
              </div>
            )}
          </div>
        )}

        {/* Footer */}
        <div className="mt-8 text-center text-sm text-gray-500">
          <p>Works completely offline • No data leaves your computer</p>
          {isCheckingUpdate && <p className="mt-1">Checking for updates...</p>}
        </div>
      </div>
    </div>
  )
}

export default App
