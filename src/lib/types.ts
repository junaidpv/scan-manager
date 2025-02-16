interface ThumbnailItem {
    title: string;
    description: string;
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
}
