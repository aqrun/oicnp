---
title: Jquery + Backbone + ejs分页
description: ''
slug: jquery-backbone-ejs-pager

taxonomies:
  categories: ['frontend', 'article']
  tags: ['backbone', '分页']
---

```html
<!--分页模板-->
<script type="text/template" id="tpl_pager">
  <a data-index="[%= index %]" title="[%= title %]"
     class="[% if(current){ %]current [% } %][%= addclass %]">[%= text %]</a>
</script>
```

```javascript
//model
var PagerModel = Backbone.Model.extend({
            defaults:{
                page:17,
                total:20
            }
        });

        //翻页视图
        window.Pager = Backbone.View.extend({
            el:"#pager",
            template: "tpl_pager",
            initialize:function(){
                _.bindAll(this,"handlePager");
                this.listenTo(this.model,"change",this.render);
            },
            events:{
                "click a" : "handlePager"
            },
            //处理页码点击事件
            handlePager: function(e){
                var $s = $(e.target);
                if($s.hasClass("dot") || $s.hasClass("current")){
                    return;
                }
                var page = parseInt($s.attr("data-index"));
                var data = {
                    action: "prosource",
                    meetid: mMeeting.meetid,
                    page: page
                };
                //请求产品列表
                $.getJSON(mMeeting.url, data,function(data){
                    proSources.add(data.data);  //保存检索到的数据
                    //获取数据成功 更新DOM
                    new EJS({element:"tpl_pro_item"}).update(mMeeting.sourceid,{data:data.data});
                    //设置页码
                    mPager.set({page:page,total:data.total});
                });
                //console.log(e.target);
            },
            render: function(){
                var page = this.model.get("page"),   //当前页码
                    total = this.model.get("total"); //总页数
                var show = 5;   //显示几个页码控制块
                var start = Math.ceil(page-show/2)<1?1:Math.ceil(page-show/2); //起始页码
                var end = Math.ceil(page+show/2-1)>total?total:Math.ceil(page+show/2-1); //结束

                var html = '';
                if(page>1){  //显示上一页
                    html += this.tpl(page-1,false,"prev","<","上一页");
                }
                if(start > 1){
                    html += this.tpl(1,false,"item",1,"第1页");  //显示第一页
                }
                if(start > 2){
                    html += this.tpl(1,false,'dot', '...','');
                }
                for(var i=start; i<=end; i++){ //显示中间页码
                    html += this.tpl(i,(page==i?true:false),'item',i,"第"+i+"页");
                }
                if(end<total-1){
                    html += this.tpl(1,false,'dot', '...','');
                }
                if(end < total){ //显示最后一页
                    html += this.tpl(total,false,"item",total,"第"+total+"页");  //显示第一页
                }
                if(page<total-1){ //显示下一页
                    html += this.tpl(page+1,false,"next",">","下一页");
                }

                this.$el.html(html);//添加dom到页面
            },
            tpl: function(index,current,addclass,text,title){
                var data = {
                    index:index,      //页码
                    current: current, //是否是当前页
                    addclass:addclass, //要添加的class
                    text:text,        //显示的文本
                    title: title      //title属性文字
                };
                return new EJS({element:this.template}).render(data);
            }
        });

        window.mPager = new PagerModel;
        var pager= new Pager({model:mPager});
        pager.render();

//////////////////
console 测试代码   mPager.set({page:3})

```
