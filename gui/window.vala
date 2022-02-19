/* window.vala
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
	[GtkTemplate (ui = "/com/innatical/id/Settings/window.ui")]
	public class Window : Adw.ApplicationWindow {
        [GtkChild]
		unowned Gtk.Stack stack;

        private Welcome welcome;
        private Settings settings;

		public Window (Adw.Application app) {
			Object (application: app);

            welcome = new Welcome ();
            welcome.to_main_ui.connect (() => load_settings ());
            
            stack.add_child (welcome);
		}

        private void load_settings () {
            if (settings != null) {
                settings.destroy ();
            }

            settings = new Settings ();
            stack.add_child (settings);
            stack.visible_child = settings;
        }
	}
}
