<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>004_LowBalance</name>
   <tag></tag>
   <elementGuidId>667aba40-768a-4635-8eb3-fce23e6abe09</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/xml; charset=utf-8</value>
   </httpHeaderProperties>
   <katalonVersion>7.8.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;soapenv:Envelope xmlns:soapenv=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:cbs=&quot;http://oss.huawei.com/business/intf/webservice/cbs&quot; xmlns:msg=&quot;http://oss.huawei.com/business/intf/webservice/cbs/msg&quot;>
   &lt;soapenv:Header/>
   &lt;soapenv:Body>
      &lt;cbs:WorkOrder>
         &lt;WorkOrderRequest>
            &lt;msg:WorkOrderType>107&lt;/msg:WorkOrderType>
            &lt;msg:SubscriberNo>987654321&lt;/msg:SubscriberNo>
            &lt;msg:operationCode>uvs&lt;/msg:operationCode>
            &lt;msg:password>?&lt;/msg:password>
            &lt;msg:AccessMode>8&lt;/msg:AccessMode>
            &lt;msg:SerialNo>14620101&lt;/msg:SerialNo>
           &lt;!--Optional:-->
            &lt;msg:ParaList>
               &lt;!--1 or more repetitions:-->
               &lt;msg:ParaItem>
                  &lt;msg:ParaName>CurrentPPSBalance&lt;/msg:ParaName>
                  &lt;msg:ParaValue>46&lt;/msg:ParaValue>
               &lt;/msg:ParaItem>
               &lt;msg:ParaItem>
                  &lt;msg:ParaName>SubCosID&lt;/msg:ParaName>
                  &lt;msg:ParaValue>500035&lt;/msg:ParaValue>
               &lt;/msg:ParaItem>
               &lt;msg:ParaItem>
                  &lt;msg:ParaName>TradeTime&lt;/msg:ParaName>
                  &lt;msg:ParaValue>20180306103026&lt;/msg:ParaValue>
               &lt;/msg:ParaItem>
               &lt;msg:ParaItem>
                  &lt;msg:ParaName>CurrentPOSBalance&lt;/msg:ParaName>
                  &lt;msg:ParaValue>0&lt;/msg:ParaValue>
               &lt;/msg:ParaItem>
            &lt;/msg:ParaList>
            &lt;msg:NotifyAccountList>
               &lt;!--1 or more repetitions:-->
               &lt;msg:NotifyAccountInfo>
                  &lt;msg:AccountCode>25047943&lt;/msg:AccountCode>
                 &lt;msg:AccountType>5231&lt;/msg:AccountType>
                  &lt;msg:NotifyType>2&lt;/msg:NotifyType>
                &lt;msg:Balance>6848512&lt;/msg:Balance>
                  &lt;msg:ExpireTime>20200705231148&lt;/msg:ExpireTime>
                  &lt;msg:ThresholdLevel>0&lt;/msg:ThresholdLevel>
                  &lt;msg:ThresholdMethod>1&lt;/msg:ThresholdMethod>
                  &lt;msg:ChangeTime>20200702235811&lt;/msg:ChangeTime>
                  &lt;msg:GroupId>0&lt;/msg:GroupId>
               &lt;/msg:NotifyAccountInfo>
            &lt;/msg:NotifyAccountList>
         &lt;/WorkOrderRequest>
      &lt;/cbs:WorkOrder>
   &lt;/soapenv:Body>
&lt;/soapenv:Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceEndpoint>${Lowbalance_Url}</soapServiceEndpoint>
   <soapServiceFunction>Work</soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>false</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.Lowbalance_Url</defaultValue>
      <description></description>
      <id>f85578bc-ce3a-4957-97aa-1a3cf130198e</id>
      <masked>false</masked>
      <name>Lowbalance_Url</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress>file:/C:/Softwares/76/LowbalanceThreshold/lowbalthreshold.wsdl.xml</wsdlAddress>
</WebServiceRequestEntity>
