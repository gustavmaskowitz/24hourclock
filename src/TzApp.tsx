import { useState, useEffect } from 'react';

const HOURS = Array.from({ length: 24 }, (_, i) => i);
const WORK_START = 8;
const WORK_END = 18;

// UTC offsets for each timezone
const TIMEZONE_OFFSETS = {
  dallas: -6,      // UTC-6
  connecticut: -5, // UTC-5
  london: 0        // UTC+0
};

// Theme configuration
const THEMES = {
  minimalist: {
    light: {
      background: '#ffffff',
      cardBg: '#f9fafb',
      cardBorder: '#e5e7eb',
      textPrimary: '#374151',
      textSecondary: '#6b7280',
      textMuted: '#9ca3af',
      segmentDefault: '#ffffff',
      segmentStroke: '#f9f9f9',
      segmentOverlap: '#007acc',
      segmentDallas: '#e5e7eb',
      segmentConnecticut: '#d1d5db',
      segmentLondon: '#f3f4f6',
      svgBg: '#f9fafb',
      svgBorder: '#e5e7eb',
      centerCircleBg: '#f8fafc',
      centerCircleBorder: '#e2e8f0',
      ringDivider: '#f9f9f9',
      nowHighlight: '#007acc',
      nowLine: '#007acc',
      clockTextOuter: '#007acc',
      clockTextMiddle: '#007acc',
      clockTextInner: '#007acc',
      clockTextTop: '#6b7280',
      buttonPrimaryBg: '#007acc',
      buttonPrimaryText: '#ffffff',
      buttonPrimaryHover: '#005a9e',
      buttonSecondaryBg: '#f3f4f6',
      buttonSecondaryText: '#6b7280',
      inputBorder: '#e5e7eb',
      inputBg: '#ffffff',
      successBg: '#f0fdf4',
      successBorder: '#bbf7d0',
      successText: '#16a34a',
      successTextDark: '#166534',
      warningBg: '#fef3c7',
      warningBorder: '#fde68a',
      warningText: '#d97706',
      warningTextDark: '#92400e',
      meetingEssential: '#22c55e',
      meetingNonEssential: '#ec4899'
    },
    dark: {
      background: '#1e1e1e',
      cardBg: '#252526',
      cardBorder: '#3c3c3c',
      textPrimary: '#d4d4d4',
      textSecondary: '#9ca3af',
      textMuted: '#6b7280',
      segmentDefault: '#2d2d30',
      segmentStroke: '#3c3c3c',
      segmentOverlap: '#007acc',
      segmentDallas: '#3a3a3c',
      segmentConnecticut: '#454545',
      segmentLondon: '#505050',
      svgBg: '#252526',
      svgBorder: '#3c3c3c',
      centerCircleBg: '#2d2d30',
      centerCircleBorder: '#3c3c3c',
      ringDivider: '#3c3c3c',
      nowHighlight: '#007acc',
      nowLine: '#007acc',
      clockTextOuter: '#007acc',
      clockTextMiddle: '#007acc',
      clockTextInner: '#007acc',
      clockTextTop: '#9ca3af',
      buttonPrimaryBg: '#007acc',
      buttonPrimaryText: '#ffffff',
      buttonPrimaryHover: '#005a9e',
      buttonSecondaryBg: '#3c3c3c',
      buttonSecondaryText: '#9ca3af',
      inputBorder: '#3c3c3c',
      inputBg: '#252526',
      successBg: '#1a2e1a',
      successBorder: '#2d4a2d',
      successText: '#4ade80',
      successTextDark: '#86efac',
      warningBg: '#3d2e1a',
      warningBorder: '#5a4520',
      warningText: '#fbbf24',
      warningTextDark: '#fcd34d',
      meetingEssential: '#22c55e',
      meetingNonEssential: '#ec4899'
    }
  },
  bold: {
    light: {
      background: '#ffffff',
      cardBg: '#ffffff',
      cardBorder: '#ec4899',
      textPrimary: '#1f2937',
      textSecondary: '#374151',
      textMuted: '#6b7280',
      segmentDefault: '#ffffff',
      segmentStroke: '#f3f4f6',
      segmentOverlap: '#ec4899',
      segmentDallas: '#fce7f3',
      segmentConnecticut: '#dbeafe',
      segmentLondon: '#ecfccb',
      svgBg: '#ffffff',
      svgBorder: '#ec4899',
      centerCircleBg: '#fdf2f8',
      centerCircleBorder: '#ec4899',
      ringDivider: '#f3f4f6',
      nowHighlight: '#ec4899',
      nowLine: '#1f2937',
      clockTextOuter: '#ec4899',
      clockTextMiddle: '#3b82f6',
      clockTextInner: '#84cc16',
      clockTextTop: '#1f2937',
      buttonPrimaryBg: '#ec4899',
      buttonPrimaryText: '#ffffff',
      buttonPrimaryHover: '#db2777',
      buttonSecondaryBg: '#f9fafb',
      buttonSecondaryText: '#374151',
      inputBorder: '#ec4899',
      inputBg: '#ffffff',
      successBg: '#f7fee7',
      successBorder: '#d9f99d',
      successText: '#65a30d',
      successTextDark: '#4d7c0f',
      warningBg: '#fef3c7',
      warningBorder: '#fde68a',
      warningText: '#d97706',
      warningTextDark: '#92400e',
      meetingEssential: '#84cc16',
      meetingNonEssential: '#ec4899'
    },
    dark: {
      background: '#1e1e1e',
      cardBg: '#2d2d30',
      cardBorder: '#c586c0',
      textPrimary: '#ffffff',
      textSecondary: '#d4d4d4',
      textMuted: '#9ca3af',
      segmentDefault: '#252526',
      segmentStroke: '#3c3c3c',
      segmentOverlap: '#c586c0',
      segmentDallas: '#3d2d3a',
      segmentConnecticut: '#2d3a4a',
      segmentLondon: '#3a3d2d',
      svgBg: '#252526',
      svgBorder: '#c586c0',
      centerCircleBg: '#3d2d3a',
      centerCircleBorder: '#c586c0',
      ringDivider: '#3c3c3c',
      nowHighlight: '#c586c0',
      nowLine: '#ffffff',
      clockTextOuter: '#c586c0',
      clockTextMiddle: '#569cd6',
      clockTextInner: '#ce9178',
      clockTextTop: '#d4d4d4',
      buttonPrimaryBg: '#c586c0',
      buttonPrimaryText: '#ffffff',
      buttonPrimaryHover: '#bd63c5',
      buttonSecondaryBg: '#3c3c3c',
      buttonSecondaryText: '#d4d4d4',
      inputBorder: '#c586c0',
      inputBg: '#252526',
      successBg: '#2a3520',
      successBorder: '#3d4a2d',
      successText: '#ce9178',
      successTextDark: '#daa520',
      warningBg: '#3d2e1a',
      warningBorder: '#5a4520',
      warningText: '#fbbf24',
      warningTextDark: '#fcd34d',
      meetingEssential: '#ce9178',
      meetingNonEssential: '#c586c0'
    }
  },
  professional: {
    light: {
      background: '#f8fafc',
      cardBg: '#ffffff',
      cardBorder: '#cbd5e1',
      textPrimary: '#1e3a8a',
      textSecondary: '#374151',
      textMuted: '#64748b',
      segmentDefault: '#ffffff',
      segmentStroke: '#f1f5f9',
      segmentOverlap: '#f59e0b',
      segmentDallas: '#dbeafe',
      segmentConnecticut: '#e0e7ff',
      segmentLondon: '#fef3c7',
      svgBg: '#ffffff',
      svgBorder: '#cbd5e1',
      centerCircleBg: '#f8fafc',
      centerCircleBorder: '#cbd5e1',
      ringDivider: '#f1f5f9',
      nowHighlight: '#0ea5e9',
      nowLine: '#1e3a8a',
      clockTextOuter: '#1e3a8a',
      clockTextMiddle: '#0ea5e9',
      clockTextInner: '#f59e0b',
      clockTextTop: '#64748b',
      buttonPrimaryBg: '#1e3a8a',
      buttonPrimaryText: '#ffffff',
      buttonPrimaryHover: '#1e40af',
      buttonSecondaryBg: '#f1f5f9',
      buttonSecondaryText: '#374151',
      inputBorder: '#cbd5e1',
      inputBg: '#ffffff',
      successBg: '#f0fdf4',
      successBorder: '#bbf7d0',
      successText: '#16a34a',
      successTextDark: '#166534',
      warningBg: '#fef3c7',
      warningBorder: '#fde68a',
      warningText: '#d97706',
      warningTextDark: '#92400e',
      meetingEssential: '#0ea5e9',
      meetingNonEssential: '#f59e0b'
    },
    dark: {
      background: '#1e293b',
      cardBg: '#334155',
      cardBorder: '#475569',
      textPrimary: '#e2e8f0',
      textSecondary: '#cbd5e1',
      textMuted: '#94a3b8',
      segmentDefault: '#334155',
      segmentStroke: '#475569',
      segmentOverlap: '#14b8a6',
      segmentDallas: '#1e3a5f',
      segmentConnecticut: '#1e2f4a',
      segmentLondon: '#3a3520',
      svgBg: '#334155',
      svgBorder: '#475569',
      centerCircleBg: '#1e293b',
      centerCircleBorder: '#475569',
      ringDivider: '#475569',
      nowHighlight: '#14b8a6',
      nowLine: '#e2e8f0',
      clockTextOuter: '#14b8a6',
      clockTextMiddle: '#22d3ee',
      clockTextInner: '#f59e0b',
      clockTextTop: '#cbd5e1',
      buttonPrimaryBg: '#14b8a6',
      buttonPrimaryText: '#ffffff',
      buttonPrimaryHover: '#0d9488',
      buttonSecondaryBg: '#475569',
      buttonSecondaryText: '#cbd5e1',
      inputBorder: '#475569',
      inputBg: '#334155',
      successBg: '#1a3a3a',
      successBorder: '#2d4a4a',
      successText: '#5eead4',
      successTextDark: '#99f6e4',
      warningBg: '#3d2e1a',
      warningBorder: '#5a4520',
      warningText: '#fbbf24',
      warningTextDark: '#fcd34d',
      meetingEssential: '#14b8a6',
      meetingNonEssential: '#f59e0b'
    }
  },
  playful: {
    light: {
      background: '#fef3c7',
      cardBg: '#ffffff',
      cardBorder: '#e9d5ff',
      textPrimary: '#7c2d12',
      textSecondary: '#92400e',
      textMuted: '#a16207',
      segmentDefault: '#ffffff',
      segmentStroke: '#fef3c7',
      segmentOverlap: '#a78bfa',
      segmentDallas: '#fed7aa',
      segmentConnecticut: '#d1fae5',
      segmentLondon: '#e9d5ff',
      svgBg: '#fffbeb',
      svgBorder: '#e9d5ff',
      centerCircleBg: '#fef3c7',
      centerCircleBorder: '#e9d5ff',
      ringDivider: '#fef3c7',
      nowHighlight: '#a78bfa',
      nowLine: '#7c2d12',
      clockTextOuter: '#ea580c',
      clockTextMiddle: '#059669',
      clockTextInner: '#a855f7',
      clockTextTop: '#92400e',
      buttonPrimaryBg: '#a855f7',
      buttonPrimaryText: '#ffffff',
      buttonPrimaryHover: '#9333ea',
      buttonSecondaryBg: '#fef3c7',
      buttonSecondaryText: '#92400e',
      inputBorder: '#e9d5ff',
      inputBg: '#ffffff',
      successBg: '#d1fae5',
      successBorder: '#a7f3d0',
      successText: '#047857',
      successTextDark: '#065f46',
      warningBg: '#fed7aa',
      warningBorder: '#fdba74',
      warningText: '#c2410c',
      warningTextDark: '#9a3412',
      meetingEssential: '#059669',
      meetingNonEssential: '#a855f7'
    },
    dark: {
      background: '#2d2d30',
      cardBg: '#3a3a3c',
      cardBorder: '#bd63c5',
      textPrimary: '#e2e8f0',
      textSecondary: '#cbd5e1',
      textMuted: '#94a3b8',
      segmentDefault: '#3a3a3c',
      segmentStroke: '#4a4a4c',
      segmentOverlap: '#bd63c5',
      segmentDallas: '#4a3520',
      segmentConnecticut: '#204a3a',
      segmentLondon: '#3d204a',
      svgBg: '#3a3a3c',
      svgBorder: '#bd63c5',
      centerCircleBg: '#2d2d30',
      centerCircleBorder: '#bd63c5',
      ringDivider: '#4a4a4c',
      nowHighlight: '#bd63c5',
      nowLine: '#e2e8f0',
      clockTextOuter: '#ce9178',
      clockTextMiddle: '#569cd6',
      clockTextInner: '#bd63c5',
      clockTextTop: '#cbd5e1',
      buttonPrimaryBg: '#bd63c5',
      buttonPrimaryText: '#ffffff',
      buttonPrimaryHover: '#a855f7',
      buttonSecondaryBg: '#4a4a4c',
      buttonSecondaryText: '#cbd5e1',
      inputBorder: '#bd63c5',
      inputBg: '#3a3a3c',
      successBg: '#1a3a2a',
      successBorder: '#2d4a3d',
      successText: '#569cd6',
      successTextDark: '#86bcf6',
      warningBg: '#3d2e1a',
      warningBorder: '#5a4520',
      warningText: '#ce9178',
      warningTextDark: '#daa520',
      meetingEssential: '#569cd6',
      meetingNonEssential: '#bd63c5'
    }
  }
};

const THEME_NAMES = {
  minimalist: 'Minimalist',
  bold: 'Bold',
  professional: 'Professional',
  playful: 'Playful'
};

export default function TimezoneClock() {
  const [meetings, setMeetings] = useState([
    { id: 1, utcHour: 16, title: 'Sync Call', essential: true }, // 16:00 UTC = 10:00 Dallas
  ]);
  const [selectedSlot, setSelectedSlot] = useState(null);
  const [currentTime, setCurrentTime] = useState(new Date());
  const [ringAssignments, setRingAssignments] = useState({
    outer: 'london',
    middle: 'connecticut',
    inner: 'dallas'
  });
  const [currentTheme, setCurrentTheme] = useState('minimalist');
  const [mode, setMode] = useState('light');

  // Get active theme colors
  const theme = THEMES[currentTheme][mode];

  useEffect(() => {
    const timer = setInterval(() => setCurrentTime(new Date()), 60000);
    return () => clearInterval(timer);
  }, []);

  const nowUTC = currentTime.getUTCHours() + currentTime.getUTCMinutes() / 60;

  // Convert from UTC to specific timezone
  const getTimezoneHour = (utcHour, timezone) => {
    const offset = TIMEZONE_OFFSETS[timezone];
    return (utcHour + offset + 24) % 24;
  };

  // Convert from one timezone to another
  const convertTimezone = (hour, fromTz, toTz) => {
    // Convert to UTC first
    const utcHour = (hour - TIMEZONE_OFFSETS[fromTz] + 24) % 24;
    // Then to target timezone
    return getTimezoneHour(utcHour, toTz);
  };

  // Get current time in a specific timezone
  const getCurrentHour = (timezone) => {
    return getTimezoneHour(nowUTC, timezone);
  };

  const isWorkHour = (hour) => hour >= WORK_START && hour < WORK_END;

  const isTimezoneWorking = (referenceHour, referenceTz, targetTz) => {
    const targetHour = convertTimezone(referenceHour, referenceTz, targetTz);
    return isWorkHour(targetHour);
  };

  const isFullOverlap = (referenceHour, referenceTz) => {
    return ['dallas', 'connecticut', 'london'].every(tz =>
      isTimezoneWorking(referenceHour, referenceTz, tz)
    );
  };

  const handleRingChange = (ring, newTimezone) => {
    // Find which ring currently has the new timezone
    const currentRingWithTimezone = Object.entries(ringAssignments).find(
      ([, tz]) => tz === newTimezone
    )?.[0];

    if (currentRingWithTimezone && currentRingWithTimezone !== ring) {
      // Swap: give the other ring our current timezone
      setRingAssignments({
        ...ringAssignments,
        [ring]: newTimezone,
        [currentRingWithTimezone]: ringAssignments[ring]
      });
    } else {
      // No conflict, just update
      setRingAssignments({
        ...ringAssignments,
        [ring]: newTimezone
      });
    }
  };

  const getHourAngle = (hour) => (hour / 24) * 360 - 90;

  const polarToCartesian = (cx, cy, r, angle) => {
    const rad = (angle * Math.PI) / 180;
    return { x: cx + r * Math.cos(rad), y: cy + r * Math.sin(rad) };
  };

  const renderSegment = (referenceHour, outerR, innerR, ring, referenceTz) => {
    // ring is 'outer', 'middle', or 'inner'
    // referenceTz tells us which timezone the referenceHour represents

    const timezone = ringAssignments[ring]; // Get which timezone is assigned to this ring
    const displayHour = convertTimezone(referenceHour, referenceTz, timezone);

    const startAngle = getHourAngle(referenceHour);
    const endAngle = getHourAngle(referenceHour + 1);

    // Check if each timezone is working
    const dallasWorking = isTimezoneWorking(referenceHour, referenceTz, 'dallas');
    const connecticutWorking = isTimezoneWorking(referenceHour, referenceTz, 'connecticut');
    const londonWorking = isTimezoneWorking(referenceHour, referenceTz, 'london');
    const fullOverlap = isFullOverlap(referenceHour, referenceTz);

    // Color based on which timezone is assigned to this ring
    let fill = theme.segmentDefault;
    if (fullOverlap) {
      fill = theme.segmentOverlap;
    } else if (timezone === 'dallas' && dallasWorking) {
      fill = theme.segmentDallas;
    } else if (timezone === 'connecticut' && connecticutWorking) {
      fill = theme.segmentConnecticut;
    } else if (timezone === 'london' && londonWorking) {
      fill = theme.segmentLondon;
    }

    const midAngle = (startAngle + endAngle) / 2;
    const labelR = (outerR + innerR) / 2;
    const labelPos = polarToCartesian(200, 200, labelR, midAngle);

    const p1 = polarToCartesian(200, 200, outerR, startAngle);
    const p2 = polarToCartesian(200, 200, outerR, endAngle);
    const p3 = polarToCartesian(200, 200, innerR, endAngle);
    const p4 = polarToCartesian(200, 200, innerR, startAngle);

    const path = `M ${p1.x} ${p1.y} A ${outerR} ${outerR} 0 0 1 ${p2.x} ${p2.y} L ${p3.x} ${p3.y} A ${innerR} ${innerR} 0 0 0 ${p4.x} ${p4.y} Z`;

    // Convert reference hour to UTC for meeting lookup
    const utcHour = Math.round((referenceHour - TIMEZONE_OFFSETS[referenceTz] + 24) % 24);
    const meeting = meetings.find(m => m.utcHour === utcHour);

    // Get hours for all three timezones for the selected slot
    const dallasHour = convertTimezone(referenceHour, referenceTz, 'dallas');
    const connecticutHour = convertTimezone(referenceHour, referenceTz, 'connecticut');
    const londonHour = convertTimezone(referenceHour, referenceTz, 'london');

    return (
      <g key={`${ring}-${referenceHour}`}>
        <path
          d={path}
          fill={fill}
          stroke={theme.segmentStroke}
          strokeWidth="1"
          className="cursor-pointer hover:brightness-95 transition-all"
          onClick={() => setSelectedSlot({ utcHour, dallasHour, connecticutHour, londonHour })}
        />
        <text
          x={labelPos.x}
          y={labelPos.y}
          textAnchor="middle"
          dominantBaseline="middle"
          fontSize="8"
          fontWeight="500"
          fill={theme.textPrimary}
          style={{ pointerEvents: 'none' }}
        >
          {String(Math.floor(displayHour)).padStart(2, '0')}
        </text>
        {ring === 'outer' && meeting && (
          <circle
            cx={labelPos.x + 10}
            cy={labelPos.y}
            r="4"
            fill={meeting.essential ? theme.meetingEssential : theme.meetingNonEssential}
            stroke={theme.background}
            strokeWidth="1"
          />
        )}
      </g>
    );
  };

  const renderNowHighlight = () => {
    const currentOuterHour = getCurrentHour(ringAssignments.outer);
    const currentOuterHourInt = Math.floor(currentOuterHour);
    const exactAngle = getHourAngle(currentOuterHour);

    const renderOutline = (hour, outerR, innerR) => {
      const startAngle = getHourAngle(hour);
      const endAngle = getHourAngle(hour + 1);

      const p1 = polarToCartesian(200, 200, outerR, startAngle);
      const p2 = polarToCartesian(200, 200, outerR, endAngle);
      const p3 = polarToCartesian(200, 200, innerR, endAngle);
      const p4 = polarToCartesian(200, 200, innerR, startAngle);

      const path = `M ${p1.x} ${p1.y} A ${outerR} ${outerR} 0 0 1 ${p2.x} ${p2.y} L ${p3.x} ${p3.y} A ${innerR} ${innerR} 0 0 0 ${p4.x} ${p4.y} Z`;

      return (
        <path
          d={path}
          fill="none"
          stroke={theme.nowHighlight}
          strokeWidth="3"
          style={{ pointerEvents: 'none' }}
        />
      );
    };

    const lineInner = polarToCartesian(200, 200, 70, exactAngle);
    const lineOuter = polarToCartesian(200, 200, 192, exactAngle);

    return (
      <>
        {renderOutline(currentOuterHourInt, 192, 156)}
        {renderOutline(currentOuterHourInt, 152, 116)}
        {renderOutline(currentOuterHourInt, 112, 70)}
        <line
          x1={lineInner.x}
          y1={lineInner.y}
          x2={lineOuter.x}
          y2={lineOuter.y}
          stroke={theme.nowLine}
          strokeWidth="2"
          strokeLinecap="round"
          style={{ pointerEvents: 'none' }}
        />
      </>
    );
  };

  const addMeeting = (utcHour, title, essential) => {
    setMeetings([...meetings, { id: Date.now(), utcHour, title, essential }]);
  };

  const removeMeeting = (id) => {
    setMeetings(meetings.filter(m => m.id !== id));
  };

  const overlapSlots = HOURS.filter(h => isFullOverlap(h, ringAssignments.outer));
  const meetingsInOverlap = meetings.filter(m => {
    const outerHour = getTimezoneHour(m.utcHour, ringAssignments.outer);
    return isFullOverlap(outerHour, ringAssignments.outer);
  });
  const meetingsOutsideOverlap = meetings.filter(m => {
    const outerHour = getTimezoneHour(m.utcHour, ringAssignments.outer);
    return !isFullOverlap(outerHour, ringAssignments.outer);
  });

  return (
    <div
      className="min-h-screen p-4 flex flex-col items-center"
      style={{
        backgroundColor: theme.background,
        transition: 'all 0.3s ease'
      }}
    >
      <h1
        className="text-2xl font-light mb-1"
        style={{ color: theme.textPrimary }}
      >
        Timezone Meeting Clock
      </h1>
      <p
        className="text-sm mb-3"
        style={{ color: theme.textSecondary }}
      >
        {ringAssignments.outer.charAt(0).toUpperCase() + ringAssignments.outer.slice(1)} (Outer) • {ringAssignments.middle.charAt(0).toUpperCase() + ringAssignments.middle.slice(1)} (Middle) • {ringAssignments.inner.charAt(0).toUpperCase() + ringAssignments.inner.slice(1)} (Inner)
      </p>

      {/* Theme Selector and Light/Dark Toggle */}
      <div className="flex gap-6 mb-4 items-center justify-center">
        {/* Theme Selector */}
        <div className="flex gap-2">
          {Object.entries(THEME_NAMES).map(([key, name]) => (
            <button
              key={key}
              onClick={() => setCurrentTheme(key)}
              style={{
                backgroundColor: currentTheme === key ? theme.buttonPrimaryBg : theme.buttonSecondaryBg,
                color: currentTheme === key ? theme.buttonPrimaryText : theme.textSecondary,
                borderRadius: '8px',
                padding: '6px 12px',
                fontSize: '12px',
                border: 'none',
                cursor: 'pointer',
                transition: 'all 0.3s ease'
              }}
            >
              {name}
            </button>
          ))}
        </div>

        {/* Light/Dark Toggle */}
        <button
          onClick={() => setMode(mode === 'light' ? 'dark' : 'light')}
          style={{
            backgroundColor: theme.cardBg,
            border: `1px solid ${theme.cardBorder}`,
            borderRadius: '20px',
            padding: '4px',
            cursor: 'pointer',
            width: '60px',
            position: 'relative',
            transition: 'all 0.3s ease'
          }}
        >
          <div
            style={{
              width: '24px',
              height: '24px',
              borderRadius: '50%',
              backgroundColor: theme.buttonPrimaryBg,
              transform: mode === 'dark' ? 'translateX(28px)' : 'translateX(0)',
              transition: 'transform 0.3s ease'
            }}
          />
        </button>
      </div>

      <div className="flex gap-4 mb-4 items-center justify-center">
        <div>
          <label
            className="text-xs block mb-1"
            style={{ color: theme.textSecondary }}
          >
            Outer Ring
          </label>
          <select
            value={ringAssignments.outer}
            onChange={(e) => handleRingChange('outer', e.target.value)}
            className="text-sm"
            style={{
              border: `1px solid ${theme.inputBorder}`,
              backgroundColor: theme.inputBg,
              color: theme.textPrimary,
              borderRadius: '6px',
              padding: '4px 8px',
              cursor: 'pointer'
            }}
          >
            <option value="dallas">Dallas</option>
            <option value="connecticut">Connecticut</option>
            <option value="london">London</option>
          </select>
        </div>

        <div>
          <label
            className="text-xs block mb-1"
            style={{ color: theme.textSecondary }}
          >
            Middle Ring
          </label>
          <select
            value={ringAssignments.middle}
            onChange={(e) => handleRingChange('middle', e.target.value)}
            className="text-sm"
            style={{
              border: `1px solid ${theme.inputBorder}`,
              backgroundColor: theme.inputBg,
              color: theme.textPrimary,
              borderRadius: '6px',
              padding: '4px 8px',
              cursor: 'pointer'
            }}
          >
            <option value="dallas">Dallas</option>
            <option value="connecticut">Connecticut</option>
            <option value="london">London</option>
          </select>
        </div>

        <div>
          <label
            className="text-xs block mb-1"
            style={{ color: theme.textSecondary }}
          >
            Inner Ring
          </label>
          <select
            value={ringAssignments.inner}
            onChange={(e) => handleRingChange('inner', e.target.value)}
            className="text-sm"
            style={{
              border: `1px solid ${theme.inputBorder}`,
              backgroundColor: theme.inputBg,
              color: theme.textPrimary,
              borderRadius: '6px',
              padding: '4px 8px',
              cursor: 'pointer'
            }}
          >
            <option value="dallas">Dallas</option>
            <option value="connecticut">Connecticut</option>
            <option value="london">London</option>
          </select>
        </div>
      </div>

      <div className="flex flex-wrap justify-center gap-3 mb-4 text-xs">
        <div className="flex items-center gap-1">
          <div
            className="w-3 h-3 rounded"
            style={{
              backgroundColor: theme.segmentOverlap,
              border: `1px solid ${theme.segmentOverlap}`,
              opacity: 0.8
            }}
          ></div>
          <span style={{ color: theme.textSecondary }}>All Overlap</span>
        </div>
        <div className="flex items-center gap-1">
          <div
            className="w-3 h-3 rounded"
            style={{
              backgroundColor: theme.segmentDallas,
              border: `1px solid ${theme.segmentDallas}`
            }}
          ></div>
          <span style={{ color: theme.textSecondary }}>Dallas</span>
        </div>
        <div className="flex items-center gap-1">
          <div
            className="w-3 h-3 rounded"
            style={{
              backgroundColor: theme.segmentConnecticut,
              border: `1px solid ${theme.segmentConnecticut}`
            }}
          ></div>
          <span style={{ color: theme.textSecondary }}>Connecticut</span>
        </div>
        <div className="flex items-center gap-1">
          <div
            className="w-3 h-3 rounded"
            style={{
              backgroundColor: theme.segmentLondon,
              border: `1px solid ${theme.segmentLondon}`
            }}
          ></div>
          <span style={{ color: theme.textSecondary }}>London</span>
        </div>
      </div>

      <svg width="400" height="400" viewBox="0 0 400 400" className="drop-shadow-lg">
        <circle
          cx="200"
          cy="200"
          r="196"
          fill={theme.svgBg}
          stroke={theme.svgBorder}
          strokeWidth="2"
        />

        {/* Outer ring */}
        {HOURS.map(h => renderSegment(h, 192, 156, 'outer', ringAssignments.outer))}

        {/* Middle ring */}
        {HOURS.map(h => renderSegment(h, 152, 116, 'middle', ringAssignments.outer))}

        {/* Inner ring */}
        {HOURS.map(h => renderSegment(h, 112, 70, 'inner', ringAssignments.outer))}

        {/* Ring dividers */}
        <circle
          cx="200"
          cy="200"
          r="156"
          fill="none"
          stroke={theme.ringDivider}
          strokeWidth="1"
          strokeDasharray="2,2"
        />
        <circle
          cx="200"
          cy="200"
          r="116"
          fill="none"
          stroke={theme.ringDivider}
          strokeWidth="1"
          strokeDasharray="2,2"
        />

        {/* Now indicator */}
        {renderNowHighlight()}

        {/* Center */}
        <circle
          cx="200"
          cy="200"
          r="40"
          fill={theme.centerCircleBg}
          stroke={theme.centerCircleBorder}
          strokeWidth="2"
        />
        <text
          x="200"
          y="188"
          textAnchor="middle"
          fontSize="8"
          fill={theme.textMuted}
        >
          NOW
        </text>
        <text
          x="200"
          y="200"
          textAnchor="middle"
          fontSize="9"
          fontWeight="600"
          fill={theme.clockTextOuter}
        >
          {String(Math.floor(getCurrentHour(ringAssignments.outer))).padStart(2, '0')}:{String(currentTime.getUTCMinutes()).padStart(2, '0')} {ringAssignments.outer.slice(0, 3).toUpperCase()}
        </text>
        <text
          x="200"
          y="211"
          textAnchor="middle"
          fontSize="9"
          fontWeight="600"
          fill={theme.clockTextMiddle}
        >
          {String(Math.floor(getCurrentHour(ringAssignments.middle))).padStart(2, '0')}:{String(currentTime.getUTCMinutes()).padStart(2, '0')} {ringAssignments.middle.slice(0, 3).toUpperCase()}
        </text>
        <text
          x="200"
          y="222"
          textAnchor="middle"
          fontSize="9"
          fontWeight="600"
          fill={theme.clockTextInner}
        >
          {String(Math.floor(getCurrentHour(ringAssignments.inner))).padStart(2, '0')}:{String(currentTime.getUTCMinutes()).padStart(2, '0')} {ringAssignments.inner.slice(0, 3).toUpperCase()}
        </text>

        {/* Top label */}
        <text
          x="200"
          y="12"
          textAnchor="middle"
          fontSize="9"
          fill={theme.clockTextTop}
          fontWeight="600"
        >
          00 {ringAssignments.outer.slice(0, 3).toUpperCase()} / {String(convertTimezone(0, ringAssignments.outer, ringAssignments.middle)).padStart(2, '0')} {ringAssignments.middle.slice(0, 3).toUpperCase()} / {String(convertTimezone(0, ringAssignments.outer, ringAssignments.inner)).padStart(2, '0')} {ringAssignments.inner.slice(0, 3).toUpperCase()}
        </text>
      </svg>

      {selectedSlot && (
        <div
          className="mt-4 p-4 w-full max-w-md"
          style={{
            backgroundColor: theme.cardBg,
            border: `1px solid ${theme.cardBorder}`,
            borderRadius: '12px',
            transition: 'all 0.3s ease'
          }}
        >
          <h3
            className="font-semibold mb-1"
            style={{ color: theme.textPrimary }}
          >
            {String(Math.floor(selectedSlot.londonHour)).padStart(2, '0')}:00 LON = {String(Math.floor(selectedSlot.connecticutHour)).padStart(2, '0')}:00 CT = {String(Math.floor(selectedSlot.dallasHour)).padStart(2, '0')}:00 DAL
          </h3>
          {(() => {
            const outerHour = getTimezoneHour(selectedSlot.utcHour, ringAssignments.outer);
            return isFullOverlap(outerHour, ringAssignments.outer) ? (
              <p
                className="text-sm mb-3"
                style={{ color: theme.successText }}
              >
                ✓ All three cities in working hours — ideal!
              </p>
            ) : (
              <p
                className="text-sm mb-3"
                style={{ color: theme.warningText }}
              >
                ⚠ {[
                  !isTimezoneWorking(outerHour, ringAssignments.outer, 'london') && 'London',
                  !isTimezoneWorking(outerHour, ringAssignments.outer, 'connecticut') && 'Connecticut',
                  !isTimezoneWorking(outerHour, ringAssignments.outer, 'dallas') && 'Dallas'
                ].filter(Boolean).join(', ')} outside working hours
              </p>
            );
          })()}

          {meetings.filter(m => m.utcHour === selectedSlot.utcHour).map(m => (
            <div
              key={m.id}
              className="flex items-center justify-between p-2 rounded mb-2"
              style={{
                backgroundColor: theme.background,
                border: `1px solid ${theme.cardBorder}`
              }}
            >
              <span className="text-sm flex items-center gap-2" style={{ color: theme.textPrimary }}>
                <span
                  className="w-2 h-2 rounded-full"
                  style={{
                    backgroundColor: m.essential ? theme.meetingEssential : theme.meetingNonEssential
                  }}
                ></span>
                {m.title}
              </span>
              <button
                onClick={() => removeMeeting(m.id)}
                className="text-xs"
                style={{ color: theme.warningText }}
              >
                ✕
              </button>
            </div>
          ))}

          <button
            onClick={() => {
              const title = prompt('Meeting title:');
              if (title) {
                const essential = confirm('Is this essential for cross-city collaboration?');
                addMeeting(selectedSlot.utcHour, title, essential);
              }
            }}
            onMouseEnter={(e) => {
              e.currentTarget.style.backgroundColor = theme.buttonPrimaryHover;
            }}
            onMouseLeave={(e) => {
              e.currentTarget.style.backgroundColor = theme.buttonPrimaryBg;
            }}
            className="mt-2 px-4 py-2 text-sm w-full"
            style={{
              backgroundColor: theme.buttonPrimaryBg,
              color: theme.buttonPrimaryText,
              borderRadius: '8px',
              border: 'none',
              cursor: 'pointer',
              transition: 'all 0.3s ease'
            }}
          >
            + Add Meeting at This Time
          </button>
        </div>
      )}

      <div className="mt-4 w-full max-w-md">
        <div
          className="p-3 mb-3"
          style={{
            backgroundColor: theme.successBg,
            border: `1px solid ${theme.successBorder}`,
            borderRadius: '12px',
            transition: 'all 0.3s ease'
          }}
        >
          <h3
            className="font-semibold text-sm mb-1"
            style={{ color: theme.successTextDark }}
          >
            Full Overlap: {overlapSlots.length} hours available
          </h3>
          <p
            className="text-xs"
            style={{ color: theme.successText }}
          >
            {overlapSlots.map(h => `${String(h).padStart(2,'0')}:00`).join(', ')} {ringAssignments.outer.charAt(0).toUpperCase() + ringAssignments.outer.slice(1)}
          </p>
          <p
            className="text-xs mt-1"
            style={{ color: theme.successText }}
          >
            {meetingsInOverlap.length} meetings in overlap window
          </p>
        </div>

        {meetingsOutsideOverlap.length > 0 && (
          <div
            className="p-3"
            style={{
              backgroundColor: theme.warningBg,
              border: `1px solid ${theme.warningBorder}`,
              borderRadius: '12px',
              transition: 'all 0.3s ease'
            }}
          >
            <h3
              className="font-semibold text-sm mb-2"
              style={{ color: theme.warningTextDark }}
            >
              ⚠ Meetings Outside Overlap
            </h3>
            {meetingsOutsideOverlap.map(m => {
              const outerHour = getTimezoneHour(m.utcHour, ringAssignments.outer);
              return (
                <div
                  key={m.id}
                  className="flex items-center justify-between text-sm py-1"
                  style={{ color: theme.warningText }}
                >
                  <span>{m.title} — {String(Math.floor(outerHour)).padStart(2,'0')}:00 {ringAssignments.outer.slice(0, 3).toUpperCase()}</span>
                  <button
                    onClick={() => removeMeeting(m.id)}
                    className="text-xs"
                    style={{ color: theme.warningTextDark, cursor: 'pointer' }}
                  >
                    Remove
                  </button>
                </div>
              );
            })}
          </div>
        )}
      </div>
    </div>
  );
}
