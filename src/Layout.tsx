import React from "react";
import {
	Page,
	Masthead,
	MastheadToggle,
	MastheadMain,
	MastheadBrand,
	MastheadContent,
	PageSidebar,
	PageSidebarBody,
	PageToggleButton,
	Toolbar,
	ToolbarContent,
	ToolbarItem,
	Nav,
	NavItem,
	NavList,
} from "@patternfly/react-core";
import BarsIcon from "@patternfly/react-icons/dist/esm/icons/bars-icon";
import { Outlet } from "react-router-dom";

export const Layout: React.FunctionComponent = () => {
	const [isSidebarOpen, setIsSidebarOpen] = React.useState(true);

	const onSidebarToggle = () => {
		setIsSidebarOpen(!isSidebarOpen);
	};

	const headerToolbar = (
		<Toolbar id="vertical-toolbar">
			<ToolbarContent>
				<ToolbarItem>header-tools</ToolbarItem>
			</ToolbarContent>
		</Toolbar>
	);

	const header = (
		<Masthead>
			<MastheadToggle>
				<PageToggleButton
					variant="plain"
					aria-label="Global navigation"
					isSidebarOpen={isSidebarOpen}
					onSidebarToggle={onSidebarToggle}
					id="vertical-nav-toggle"
				>
					<BarsIcon />
				</PageToggleButton>
			</MastheadToggle>
			<MastheadMain>
				<MastheadBrand href="https://patternfly.org" target="_blank">
					Logo
				</MastheadBrand>
			</MastheadMain>
			<MastheadContent>{headerToolbar}</MastheadContent>
		</Masthead>
	);

	const [activeItem, setActiveItem] = React.useState(0);
	const onSelect = (
		_event: React.FormEvent<HTMLInputElement>,
		result: { itemId: number | string },
	) => {
		setActiveItem(result.itemId as number);
	};

	const sidebar = (
		<PageSidebar isSidebarOpen={isSidebarOpen} id="vertical-sidebar">
			<PageSidebarBody>
				<Nav
					onSelect={onSelect}
					aria-label="Default global"
					ouiaId="DefaultNav"
				>
					<NavList>
						<NavItem
							preventDefault
							id="nav-default-link1"
							to="#nav-default-link1"
							itemId={0}
							isActive={activeItem === 0}
						>
							{"Default Link 1"}
						</NavItem>
					</NavList>
				</Nav>
			</PageSidebarBody>
		</PageSidebar>
	);

	return (
		<Page header={header} sidebar={sidebar}>
			<Outlet />
		</Page>
	);
};
