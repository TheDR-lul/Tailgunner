import { useState, useEffect, useRef } from 'react';

interface EditableNumberInputProps {
  value: number;
  onChange: (value: number) => void;
  min?: number;
  max?: number;
  step?: number;
  decimals?: number;
  suffix?: string;
  style?: React.CSSProperties;
}

export function EditableNumberInput({
  value,
  onChange,
  min = 0,
  max = Infinity,
  decimals = 1,
  suffix = '',
  style
}: EditableNumberInputProps) {
  const [editValue, setEditValue] = useState(value.toFixed(decimals));
  const [isFocused, setIsFocused] = useState(false);
  const inputRef = useRef<HTMLInputElement>(null);

  // Update edit value when prop changes (but only if not focused)
  useEffect(() => {
    if (!isFocused) {
      setEditValue(value.toFixed(decimals));
    }
  }, [value, decimals, isFocused]);

  const handleFocus = () => {
    setIsFocused(true);
    // Select all text on focus for easy editing
    if (inputRef.current) {
      inputRef.current.select();
    }
  };

  const handleBlur = () => {
    setIsFocused(false);
    
    // Parse and validate the value
    const parsed = parseFloat(editValue);
    
    if (isNaN(parsed)) {
      // Reset to original value if invalid
      setEditValue(value.toFixed(decimals));
      return;
    }
    
    // Clamp to min/max
    const clamped = Math.max(min, Math.min(max, parsed));
    
    // Update if changed
    if (clamped !== value) {
      onChange(clamped);
    }
    
    // Format the display value
    setEditValue(clamped.toFixed(decimals));
  };

  const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    // Allow any input while editing (including partial numbers like "0.")
    setEditValue(e.target.value);
  };

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter') {
      inputRef.current?.blur();
    } else if (e.key === 'Escape') {
      setEditValue(value.toFixed(decimals));
      inputRef.current?.blur();
    }
  };

  return (
    <input
      ref={inputRef}
      type="text"
      inputMode="decimal"
      value={isFocused ? editValue : editValue + suffix}
      onChange={handleChange}
      onFocus={handleFocus}
      onBlur={handleBlur}
      onKeyDown={handleKeyDown}
      onWheel={(e) => e.currentTarget.blur()}
      style={style}
    />
  );
}

