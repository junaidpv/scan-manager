interface ThumbnailItem {
    src: string;
    alt?: string;
    title?: string;
}

/**
 * Represents a project items
 */
interface ProjectItem {
    /**
     * The name of the project item.
     */
    name: string;

    /**
     * The location where the item was scanned.
     */
    scan_location: string;

    /**
     * A brief description of the project.
     */
    description: string;

    /**
     * The timestamp when the project item was created.
     */
    created_at: number;

    /**
     * The timestamp when the project item was last updated.
     */
    updated_at: number;
}

interface SidebarItemInfo {
    title: string;
    description: string;
    button_color: string;
    icon: string | null;
    onclick: Function | null | undefined
}

interface ChildWindow {
    name: string;
    title: string;
    url: string;
    created_callback?: Function;
    close_callback?: Function;
    error_callback?: Function;
}


interface StepSettings {
    deskew_mode: string;
    deskew_value: number;
    frame_ratio_width: number;
    frame_ratio_height: number;
    rotate_angle: number;
    frame_settings: FrameSettings;
}

interface FrameSettings {
    start_x: number;
    start_y: number;
    end_x: number | null;
    end_y: number | null;
}

interface ImageInfo {
    width: number;
    height: number;
    size: number;
    format: string;
    path: string;
    name: string;
}