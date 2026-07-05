# Copyright (C) 2026  Antonio-Miguel Corbi Bellot
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with this program.  If not, see <http://www.gnu.org/licenses/>.

# List this help
@default:
    just --list

# Update packages
@update:
    cargo update

# Where are we?
system-info:
  @echo "This is an {{arch()}} machine".

# Count lines of code
@sloc:
    tokei

# Cargo check
c:
    cargo c

# Cargo buid
b:
    cargo b

# Cargo buid debug
bd:
    cargo b --features debug

# Cargo run
r:
    cargo r

# Cargo run debug
rd:
    cargo r --features debug
