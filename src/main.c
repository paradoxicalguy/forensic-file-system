#include <stdint.h>
#include <time.h>
#include <string.h>
#include <stdlib.h>
#include <stdio.h>

#define FILETYPE_FILE 1
#define FILETYPE_DIR  2
#define FS_MAGIC 0xF0F03410


typedef struct {
    // identity
    uint32_t magic_number;
    uint32_t version;

    // size info
    uint32_t block_size;
    uint32_t total_blocks;
    uint64_t fs_size;

    // inode info
    uint32_t inode_count;
    uint32_t free_inodes;
    uint32_t first_inode_block;
    uint32_t inode_blocks;
    uint32_t inode_bitmap_block;

    // block allocation
    uint32_t free_blocks;
    uint32_t first_data_block;
    uint32_t bitmap_block; 

    // directory structure
    uint32_t root_inode;

    // forensic timestamps
    int64_t created_time;
    int64_t last_mount_time;
    int64_t last_write_time;
    
    // forensic features
    uint32_t mount_count;
    uint32_t state;

    // backups
    uint32_t backup_superblock[5];

    // reserved 
    uint8_t reserved[128];

} superblock_t;

typedef struct {
    // core 
    uint32_t inode_number;
    uint32_t file_type;
    uint64_t size;

    uint32_t direct_blocks [12];
    uint32_t indirect_block; 

    //timestamps
    int64_t created_time;
    int64_t modified_time;
    int64_t accessed_time;
    

    // forensics
    int64_t deleted_time;
    uint32_t is_deleted; 
    uint32_t tamper_flag;

    // optional stuff
    uint32_t owner_id;
    uint32_t permissions;
    uint32_t link_count;
} inode_t;

////////////////////////////////////////////////


superblock_t create_superblock(uint32_t block_size, uint32_t total_blocks);
inode_t create_inode (uint32_t inode_number, uint32_t file_type, uint32_t permissions, uint32_t owner_id);

int main() {
    superblock_t sb = create_superblock (4096, 5000);
    inode_t inode = create_inode (1, FILETYPE_DIR, 0777, 0);
    create_disk_image("disk.img", 4096 * 5000);
    write_superblock("disk.img", &sb, 4096, 0);
    init_block_bitmap();
    init_inode_bitmap();
    init_inode_table(&sb);
    return 0;   
}

superblock_t create_superblock (uint32_t block_size, uint32_t total_blocks) {
    superblock_t sb;

    sb.magic_number = FS_MAGIC;
    sb.version = 1;

    sb.block_size = block_size;
    sb.total_blocks = total_blocks;
    sb.fs_size = (uint64_t)block_size * total_blocks;

    sb.inode_count = 320;
    sb.free_inodes = 320;
    sb.inode_blocks = 8;

    sb.first_inode_block = 3;
    sb.free_blocks = total_blocks - 11;
    sb.first_data_block = 11;
    sb.bitmap_block = 1; 

    sb.root_inode = 1;
    sb.inode_bitmap_block = 2;

    sb.created_time = time(NULL);
    sb.last_mount_time = sb.created_time;
    sb.last_write_time = sb.created_time; 

    sb.mount_count = 0;
    sb.state = 0; 

    for (int i = 0; i<5; i++ ) {
        sb.backup_superblock[i] = 0;
    }

    memset(sb.reserved, 0, sizeof(sb.reserved));
    return sb;

}
inode_t create_inode (uint32_t inode_number, uint32_t file_type, uint32_t permissions, uint32_t owner_id) {
    inode_t inode;

    inode.inode_number = inode_number;
    inode.file_type = file_type;
    inode.size = 0;


    memset(inode.direct_blocks, 0, sizeof(inode.direct_blocks));
    inode.indirect_block = 0;

    inode.permissions = permissions;
    inode.owner_id = owner_id;
    inode.link_count = 1;

    inode.deleted_time = 0;
    inode.is_deleted = 0;
    inode.tamper_flag = 0;

    int64_t now = time(NULL);
    inode.created_time = now;
    inode.modified_time = now;
    inode.accessed_time = now;

    return inode;

};

int create_disk_image(const char *disk, uint64_t size) {
    FILE *fp = fopen(disk, "wb");
    if (!fp) {
        perror ("failed to create disk image");
        return -1;
    }
    if (fseek(fp, size - 1, SEEK_SET) != 0) {
        perror("failed to seek");
        fclose(fp);
        return -1;
    }   
    fputc(0, fp);
    fclose (fp);
    return 0;
}

int write_superblock (const char *disk, superblock_t *sb, uint32_t block_size, uint32_t block_number) {
    FILE *fp = fopen(disk, "r+b");
    if (!fp) {
        perror ("failed to open disk image");
        return -1;
    }
    if (fseek(fp, block_number * block_size, SEEK_SET) != 0) {
        perror("failed to seek");
        fclose(fp);
        return -1;
    }
    fwrite(sb, sizeof(superblock_t), 1, fp);
    fclose(fp);
    return 0;
}

int read_block (const char *disk, uint8_t *buffer, uint32_t block_size, uint32_t block_number) {
    FILE *fp = fopen (disk, "rb");
    if (!fp) {
        perror ("failed to open disk");
        return -1;
    }

    if (fseek (fp, (long)block_number * block_size, SEEK_SET)!=0) {
        perror("faile to seek");
        return -1; 
    }

    if (fread(buffer, block_size, 1, fp) !=1) {
        perror("faile to read block");
        fclose(fp);
        return -1;
    }
    fclose(fp);
    return 0;
}

int write_block_bitmap (const char *disk, uint8_t *bitmap, uint32_t block_size, uint32_t block_number) {
    FILE *fp = fopen(disk, "r+b");
    if (!fp) {
        perror("failed to open disk image");
        return -1;
    }
    if (fseek(fp, block_size * block_number, SEEK_SET)!=0) {
        perror("failed to seek");
        fclose(fp);
        return -1;
    }
    fwrite(bitmap, block_size, 1, fp);
    fclose(fp);
    return 0;
}

void bitmap_set (uint8_t *bitmap, uint32_t block_number) {
    uint32_t byte_index = block_number / 8;
    uint32_t bit_index  = block_number % 8;
    bitmap[byte_index] |= (1 << bit_index);
}

int bitmap_test(uint8_t *bitmap, uint32_t block_number) {
    uint32_t byte = block_number / 8;
    uint32_t bit  = block_number % 8;
    return (bitmap[byte] >> bit) & 1;
}

void bitmap_clear(uint8_t *bitmap, uint32_t block_number) {
    uint32_t byte = block_number / 8;
    uint32_t bit  = block_number % 8;
    bitmap[byte] &= ~(1 << bit);
}

int bitmap_find_free(uint8_t *bitmap, uint32_t max_bits) {
    for (uint32_t i=0; i<max_bits; i++) {
        if (!bitmap_test(bitmap, i)) {
            return (int)i; 
        }
    }
    return -1;
}


int init_block_bitmap() {
    uint8_t block_bitmap[4096];
    memset(block_bitmap, 0, sizeof(block_bitmap));

    for (int i = 0; i <=10; i++) {
        bitmap_set(block_bitmap, i);
    }
    write_block_bitmap("disk.img", block_bitmap, 4096, 1);  
    return 0;
}

int init_inode_bitmap () {
    uint8_t inode_bitmap[4096];
    memset(inode_bitmap, 0, sizeof(inode_bitmap));
    bitmap_set (inode_bitmap, 0);
    bitmap_set (inode_bitmap, 1);
    write_block_bitmap("disk.img", inode_bitmap, 4096, 2);
    return 0;
}

int init_inode_table (superblock_t *sb) {
    uint32_t total_size = sb->inode_blocks * sb->block_size;

    uint8_t *table = malloc((size_t)total_size);
    if (!table) {
        perror ("malloc failed for inode table");
        return -1;
    }
    memset(table, 0, total_size);

    inode_t root = create_inode(1, FILETYPE_DIR, 0755, 0);
    memcpy(table, &root, sizeof(inode_t));

    FILE *fp = fopen("disk.img", "r+b");
    if (!fp) {
        perror("failed to open disk");
        free(table);
        return -1;
    }

    for (uint32_t i= 0; i < sb->inode_blocks; i++) {
        uint32_t block_number = sb->first_inode_block + i;
        uint32_t offset = i * sb->block_size;

        if (fseek(fp, (long)block_number * sb->block_size, SEEK_SET)!=0) {
            perror("fseek failed while writing inode table");
            fclose(fp);
            free(table);
            return -1;
        }
        if (fwrite(table+offset, sb->block_size, 1, fp) !=1) {
            perror("fwrite failed while writing inode table");
            fclose(fp);
            free(table);
            return -1;
        }
    }  
    fclose(fp);
    free(table);
    return 0;
}

int alloc_block (superblock_t *sb) {
    uint8_t bitmap [4096];

    if (read_block("disk.img", bitmap, sb->block_size, sb->bitmap_block)!=0) {
        return -1;
    }

    int free_block = bitmap_find_free(bitmap, sb->total_blocks);
    if (free_block < 0) {
        printf("no free blocks");
        return -1;
    }

    bitmap_set(bitmap, free_block);

    if (write_block_bitmap("disk.img", bitmap, sb->block_size, sb->bitmap_block)!=0) {
        return -1;
    }

    sb->free_blocks--;
    return free_block;
}