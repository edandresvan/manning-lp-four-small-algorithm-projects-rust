/// Number of disks to move in the tower of Hanoi problem.
const NUM_DISKS: usize = 12;

/// Represents a disk of the tower of Hanoi problem.
#[derive(Copy, Clone, Debug, PartialEq)]
enum DiskSlot {
  /// Size of the disk.
  DiskSize(u32),
  Empty,
}

impl std::fmt::Display for DiskSlot {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    // Use the format desired by the user.
    // Source: https://stackoverflow.com/questions/72544241/how-to-implement-display-while-respecting-width-fill-and-alignment
    match self {
      Self::DiskSize(size) => size.fmt(f),
      Self::Empty => "0".fmt(f),
    }
  }
}

/// Moves one disk from the given origin post to the destination post.
///
/// # Arguments
///
/// * `posts`: Two-dimensional array containinig the three poles or towers of Hanoi.
/// * `from_post`: Index of the origin post of the disk.
/// * `to to_post`: Index of the destination post to which the disk will be moved.
///
/// # Remarks
///
/// The `posts` array will have the new arrangement of disks after this move.
fn move_disk(
  posts: &mut [[DiskSlot; NUM_DISKS]; 3],
  from_post: usize,
  to_post: usize,
) {
  // Find the first occupied slot in the origin post.
  let mut from_disk_index: usize = 0;
  // Look for an occupied slot from the start
  for disk_index in 0..NUM_DISKS {
    if posts[from_post][disk_index] != DiskSlot::Empty {
      from_disk_index = disk_index;
      break;
    }
  }

  // Find the last empty row in the destination post.
  let mut to_disk_index: usize = NUM_DISKS - 1;
  // Look for an empty slot from the end
  for disk_index in (0..NUM_DISKS).rev() {
    if posts[to_post][disk_index] == DiskSlot::Empty {
      to_disk_index = disk_index;
      break;
    }
    to_disk_index -= 1;
  }

  //  println!("{}", posts[from_post][from_disk_index]);
  // Swap the values at those positions.
  (
    posts[from_post][from_disk_index],
    posts[to_post][to_disk_index],
  ) = (
    posts[to_post][to_disk_index],
    posts[from_post][from_disk_index],
  );
}

/// Moves the given number of disks from the origin post to the destination post.
///
/// # Arguments
///
/// * `posts`: Two-dimensional array containinig the three poles or towers of Hanoi.
/// * `num_to_move`: Number of disks to move.
/// * `from_post`: Index of the origin post of the disk.
/// * `to to_post`: Index of the destination post to which the disk will be moved.
/// * `temp to_post`: Index of the post serving as a temporary storage.
///
/// # Remarks
///
/// The `posts` array will have the new arrangement of disks after moving the disks.
fn move_disks(
  posts: &mut [[DiskSlot; NUM_DISKS]; 3],
  num_to_move: usize,
  from_post: usize,
  to_post: usize,
  temp_post: usize,
) {
  // We need to move more than one disk
  if num_to_move > 1 {
    // Move N - 1 disks from from_post to temp_post using the to_post as temporary storage.
    move_disks(posts, num_to_move - 1, from_post, temp_post, to_post);
  }
  // Move the top disk from_post to to_post. (
  move_disk(posts, from_post, to_post);

  // We need to move more than one disk
  if num_to_move > 1 {
    // Move N - 1 disks from temp_post to to_post using from_post as temporary storage.
    move_disks(posts, num_to_move - 1, temp_post, to_post, from_post);
  }
}

/// Shows in scren the given two-dimensional array of posts.
///
/// # Arguments
///
/// * `posts`: Two-dimensional array containinig the three poles or towers of Hanoi.

fn draw_posts(posts: &[[DiskSlot; NUM_DISKS]; 3]) {
  let num_rows: usize = posts.len();
  let num_columns: usize = posts[0].len();

  // Amount of dashes to show as the base of the figure
  let mut dashes: usize = 0;

  // Iterate over the poles (columns)
  for index_post in 0..num_columns {
    // Stores the elements of each row
    let mut elements: Vec<String> = Vec::new();
    // Iterate over the slots (rows)
    for index_disk in 0..num_rows {
      // Get the contents of each slot
      elements.push(format!("{:>2} ", posts[index_disk][index_post]));
    }
    // Separate each element
    let line: String = format!(" |{} |", elements.join("￤"));
    // The amount of dashes will equal to the largest line
    dashes = std::cmp::max(dashes, line.chars().count());
    // Print the line
    println!("{}", line);
  }
  // Print the base of the figure.
  println!(" {}", "=".repeat(dashes + 1));
}

fn main() {
  // Make three posts (columns) with NUM_DISKS empty slots in each (rows).
  let mut posts = [[DiskSlot::Empty; NUM_DISKS]; 3];

  for i in (0..NUM_DISKS).rev() {
    posts[0][i] = DiskSlot::DiskSize((i + 1) as u32);
  }

  println!("Tower of Hanoi at the start\n");
  draw_posts(&posts);
  // Move the disks.
  move_disks(&mut posts, NUM_DISKS, 0, 2, 1);

  println!();
  println!("Tower of Hanoi at the end\n");
  draw_posts(&posts);
}
