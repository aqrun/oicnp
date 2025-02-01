"use client"

import { usePathname } from 'next/navigation';
import { ChevronRight, type LucideIcon } from "lucide-react"

import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible"
import {
  SidebarGroup,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  SidebarMenuSub,
  SidebarMenuSubButton,
  SidebarMenuSubItem,
} from "@/components/ui/sidebar";
import { MenuTreeItem } from '@/services/types';
import {
  Icon,
} from '@/components';

export function NavMain({
  items,
}: {
  items: MenuTreeItem[];
}) {
  const pathname = usePathname();

  console.log('pathname', pathname);

  return (
    <SidebarGroup>
      {/* <SidebarGroupLabel>Platform</SidebarGroupLabel> */}
      <SidebarMenu>
        {items.map((item) => {
          if (!item?.children?.length) {
            return (
              <SidebarMenuButton asChild key={item.id}>
                <a href={item.path}>
                  <Icon icon={item.icon} />
                  <span>{item.name}</span>
                </a>
              </SidebarMenuButton>
            );
          }

          const open = item.children?.find((n) => {
            return n?.path?.includes(pathname);
          });

          return (
            <Collapsible
              key={item.id}
              asChild
              defaultOpen={Boolean(open)}
              className="group/collapsible"
            >
              <SidebarMenuItem>
                <CollapsibleTrigger asChild>
                  <SidebarMenuButton tooltip={item?.name}>
                    {item.icon && <Icon icon={item.icon} />}
                    <span>{item?.name}</span>
                    <ChevronRight className="ml-auto transition-transform duration-200 group-data-[state=open]/collapsible:rotate-90" />
                  </SidebarMenuButton>
                </CollapsibleTrigger>
                <CollapsibleContent>
                  <SidebarMenuSub>
                    {item.children?.map((subItem) => (
                      <SidebarMenuSubItem key={subItem.id}>
                        <SidebarMenuSubButton asChild>
                          <a href={subItem.path}>
                            {subItem.icon && <Icon icon={subItem.icon} />}
                            <span>{subItem.name}</span>
                          </a>
                        </SidebarMenuSubButton>
                      </SidebarMenuSubItem>
                    ))}
                  </SidebarMenuSub>
                </CollapsibleContent>
              </SidebarMenuItem>
            </Collapsible>
          );
        })}
      </SidebarMenu>
    </SidebarGroup>
  )
}
