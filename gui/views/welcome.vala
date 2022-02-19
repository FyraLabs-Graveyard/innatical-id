/* views/welcome.vala
 *
 * Copyright 2022 Innatical, LLC
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

 namespace InnaticalID {
	[GtkTemplate (ui = "/com/innatical/id/Settings/welcome.ui")]
	public class Welcome : Adw.Bin {
        [GtkChild]
		unowned Adw.StatusPage welcome_home;
        [GtkChild]
		unowned Gtk.Button main_ui_btn;

        public signal void to_main_ui ();

		public Welcome () {
			Object ();

            var image = new Gtk.Image.from_resource ("/com/innatical/id/Settings/earth-americas.svg").get_paintable();

            main_ui_btn.clicked.connect (() => {
                to_main_ui ();
            });

            main_ui_btn.sensitive = true;
            
            welcome_home.set_paintable (image);
		}
	}
}
