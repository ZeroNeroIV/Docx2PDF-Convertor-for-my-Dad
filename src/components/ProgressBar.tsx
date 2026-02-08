interface ProgressBarProps {
  progress: number
}

export default function ProgressBar({ progress }: ProgressBarProps) {
  const clampedProgress = Math.max(0, Math.min(100, progress))
  
  return (
    <div className="w-full">
      <div className="flex justify-between mb-1">
        <span className="text-sm font-medium text-gray-700">Overall Progress</span>
        <span className="text-sm font-medium text-primary-600">{Math.round(clampedProgress)}%</span>
      </div>
      <div className="w-full bg-gray-200 rounded-full h-2.5">
        <div
          className="bg-primary-600 h-2.5 rounded-full transition-all duration-300 ease-out"
          style={{ width: `${clampedProgress}%` }}
        />
      </div>
    </div>
  )
}
